use super::{
    room::Room,
    socket::{Event, Socket},
};
use rocket::tokio::{
    self,
    sync::Mutex,
    time::{sleep, Duration},
};
use std::sync::Arc;

lazy_static! {
    static ref ROOMS: Arc<Mutex<Vec<Room>>> = Arc::new(Mutex::new(Vec::new()));
}

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
