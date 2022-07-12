use futures_util::{SinkExt, StreamExt};
use rocket::tokio::{
    self,
    net::{TcpListener, TcpStream},
};
use std::{net::SocketAddr, time::Duration};
use tokio_tungstenite::{accept_async, tungstenite::Error};
use tungstenite::{Message, Result};

async fn accept_connection(peer: SocketAddr, stream: TcpStream) {
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed | Error::Protocol(_) | Error::Utf8 => (),
            err => error!("Error processing connection: {}", err),
        }
    }
}

async fn handle_connection(peer: SocketAddr, stream: TcpStream) -> Result<()> {
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    info!("New WebSocket connection: {}", peer);
    let (mut ws_sender, mut ws_receiver) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(1000));

    loop {
        tokio::select! {
            msg = ws_receiver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        if msg.is_text() ||msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                ws_sender.send(Message::Text("tick".to_owned())).await?;
            }
        }
    }

    Ok(())
}

pub async fn start_ws() {
    let addr = "0.0.0.0:7878";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    info!("WS Listening on: {}", addr);

    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream
            .peer_addr()
            .expect("connected streams should have a peer address");

        tokio::spawn(accept_connection(peer, stream));
    }
}
