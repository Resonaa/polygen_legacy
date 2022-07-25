use futures_util::{SinkExt, StreamExt};
use rocket::{
    serde::{
        json::{self, serde_json, Value},
        Deserialize, Serialize,
    },
    tokio::{
        self,
        net::{TcpListener, TcpStream},
        sync::{
            broadcast::{self, Receiver, Sender},
            Mutex,
        },
    },
};
use std::{error::Error, sync::Arc};
use tokio_tungstenite::{accept_async, tungstenite::Message};

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Event {
    pub id: i32,
    pub name: String,
    pub dat: Value,
}

impl Event {
    pub fn new(
        id: i32,
        name: impl ToString,
        dat: impl Serialize,
    ) -> Result<Self, serde_json::error::Error> {
        Ok(Self {
            id,
            name: name.to_string(),
            dat: json::to_value(dat)?,
        })
    }

    pub fn from(id: i32, value: &str) -> Result<Self, serde_json::error::Error> {
        #[derive(Serialize, Deserialize)]
        #[serde(crate = "rocket::serde")]
        struct FromEvent {
            name: String,
            dat: Value,
        }

        let value: FromEvent = json::from_str(value)?;

        Ok(Self {
            id,
            name: value.name,
            dat: value.dat,
        })
    }
}

pub struct Socket {
    s: Sender<Event>,
}

impl Socket {
    pub async fn new(
        addr: &'static str,
        handler: impl Fn(Event) -> Option<Event> + Send + Sync + Copy + 'static,
    ) -> Self {
        let listener = TcpListener::bind(addr).await.unwrap();
        info!("WS Listening on: {}", addr);

        let (s, _) = broadcast::channel(20);
        let ps = s.clone();

        let id = Arc::new(Mutex::new(0));
        tokio::spawn(async move {
            while let Ok((stream, _)) = listener.accept().await {
                let mut id = id.lock().await;
                *id += 1;
                tokio::spawn(handle_connection(
                    stream,
                    ps.clone(),
                    ps.subscribe(),
                    *id,
                    handler,
                ));
            }
        });

        Self { s }
    }

    pub fn send(&self, event: Event) {
        self.s.send(event).unwrap_or_default();
    }
}

async fn handle_connection(
    stream: TcpStream,
    sender: Sender<Event>,
    mut receiver: Receiver<Event>,
    id: i32,
    handler: impl Fn(Event) -> Option<Event> + Copy,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        tokio::select! {
            Some(Ok(msg)) = ws_receiver.next() =>
                match msg {
                    Message::Text(msg) => if let Some(response) = Event::from(id, &msg).ok().and_then(handler) {
                        sender.send(response)?;
                    },
                    Message::Close(_) => {
                        sender.send(Event::new(0, "close", id)?)?;
                        break;
                    }
                    _ => ()
                }
            ,
            Ok(msg) = receiver.recv() =>
                if msg.id == id || msg.id == 0 {
                    ws_sender.send(Message::Text(json::to_string(&msg)?)).await?;
                }
        }
    }

    Ok(())
}
