use super::event::{Event, EventName};
use futures_util::{SinkExt, StreamExt};
use rocket::{
    serde::json,
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

pub struct Socket {
    s: Sender<Event>,
}

impl Socket {
    pub async fn new<T: futures_util::Future<Output = Vec<Event>> + Send + Sync + 'static>(
        addr: &'static str,
        handler: impl Fn(Event) -> T + Send + Sync + Copy + 'static,
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

async fn handle_connection<T: futures_util::Future<Output = Vec<Event>>>(
    stream: TcpStream,
    sender: Sender<Event>,
    mut receiver: Receiver<Event>,
    id: i32,
    handler: impl Fn(Event) -> T + Copy + Send,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let ws_stream = accept_async(stream).await?;
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();

    loop {
        tokio::select! {
            Some(Ok(msg)) = ws_receiver.next() =>
                match msg {
                    Message::Text(msg) => if let Ok(event) = Event::from(id, &msg) {
                        for response in handler(event).await {
                            sender.send(response)?;
                        }
                    },
                    _ => {
                        for response in handler(Event::new(id, EventName::Close, ())?).await {
                            sender.send(response)?;
                        }
                        break;
                    }
                }
            ,
            Ok(msg) = receiver.recv() =>
                if msg.id == id || msg.id == 0 {
                    match msg.name {
                        EventName::Abort => {
                            for response in handler(Event::new(id, EventName::Close, ())?).await {
                                ws_sender.send(Message::Text(json::to_string(&response)?)).await?;
                            }
                            break;
                        }
                        EventName::ClearExisted  => if json::from_value::<i32>(msg.dat).unwrap_or(id) != id {
                            for response in handler(Event::new(id, EventName::Close, ())?).await {
                                ws_sender.send(Message::Text(json::to_string(&response)?)).await?;
                            }
                            break;
                        }
                        _ => {
                            ws_sender.send(Message::Text(json::to_string(&msg)?)).await?;
                        }
                    }
                }
        }
    }

    Ok(())
}
