use super::{
    event::{Event, EventName},
    room::Room,
    socket::Socket,
};
use hashbrown::HashMap;
use log::info;
use rocket::{
    serde::{
        json::{self, Value},
        Deserialize, Serialize,
    },
    tokio::sync::Mutex,
};

mod generator;
pub mod land;
pub mod map;

lazy_static! {
    pub static ref ROOMS: Mutex<Vec<Room>> = Mutex::new(Vec::new());
    pub static ref IDENTITIES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    pub static ref PLAYERS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
}

pub async fn remove_player(username: &str) {
    IDENTITIES.lock().await.remove(username);

    for room in &mut *ROOMS.lock().await {
        if room.players.remove(username).is_some() {
            return;
        }
    }
}

async fn identify(value: Value) -> Result<String, ()> {
    #[derive(Serialize, Deserialize)]
    #[serde(crate = "rocket::serde")]
    struct FromIdentification {
        username: String,
        identity: String,
    }

    let identification: FromIdentification = json::from_value(value).map_err(|_| ())?;

    for (username, identity) in IDENTITIES.lock().await.iter() {
        if identity == &identification.identity {
            return Ok(username.to_string());
        }
    }

    Err(())
}

pub async fn game() {
    let _socket = Socket::new("0.0.0.0:7878", |event| async move {
        info!("{:?}", event);

        let mut rooms = ROOMS.lock().await;
        if rooms.len() <= 2 {
            rooms.push(Room::new(0));
            rooms.push(Room::new(1));
        }

        if event.name == EventName::Identify {
            if let Ok(username) = identify(event.dat).await {
                PLAYERS.lock().await.insert(event.id, username);
                return Event::new(event.id, EventName::Message, "身份验证成功").ok();
            } else {
                return Event::new(event.id, EventName::Message, "身份验证失败").ok();
            }
        }

        let username = match PLAYERS.lock().await.get(&event.id) {
            None => return Event::new(event.id, EventName::Message, "未进行身份验证").ok(),
            Some(username) => username,
        }
        .to_string();

        match event.name {
            EventName::Close => {
                remove_player(&username).await;
                PLAYERS.lock().await.remove(&event.id);
                None
            }
            EventName::Message => Event::new(
                event.id,
                EventName::Message,
                format!("hello, {}!", event.id),
            )
            .ok(),

            _ => None,
        }
    })
    .await;
}
