use super::{
    room::Room,
    socket::{Event, Socket},
};
use rocket::tokio::{
    self,
    sync::Mutex,
    time::{sleep, Duration},
};

mod generator;
pub mod land;
pub mod map;

lazy_static! {
    pub static ref ROOMS: Mutex<Vec<Room>> = Mutex::new(Vec::new());
}

pub async fn game() {
    let socket = Socket::new("0.0.0.0:7878", |event| async move {
        info!("got {:?}", event);
        ROOMS.lock().await.push(Room::default());
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
