#![allow(unused)]

use crate::socket::{Event, Socket};
use rocket::tokio::{
    self,
    time::{sleep, Duration},
};

mod generator;
mod map;

pub async fn game() {
    let socket = Socket::new("0.0.0.0:7878", |event| {
        info!("got {:?}", event);
        Event::new(event.id, "echo", format!("hello, {}!", event.id)).ok()
    })
    .await;

    tokio::spawn(async move {
        loop {
            socket.send(Event::new(0, "broadcast", "b").unwrap());
            sleep(Duration::from_secs(1)).await;
        }
    });
}
