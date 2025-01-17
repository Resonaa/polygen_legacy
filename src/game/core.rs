use super::{
    event::{Event, EventName},
    room::Room,
    socket::Socket,
};
use hashbrown::HashMap;
use log::info;
use rocket::{
    serde::{
        json::{self, json, Value},
        Deserialize, Serialize,
    },
    tokio::sync::Mutex,
};

mod generator;
pub mod land;
pub mod map;

lazy_static! {
    pub static ref ROOMS: Mutex<Vec<Room>> = Mutex::new(vec![Room::new("161")]);
    pub static ref IDENTITIES: Mutex<HashMap<String, String>> = Mutex::new(HashMap::new());
    pub static ref PLAYERS: Mutex<HashMap<i32, String>> = Mutex::new(HashMap::new());
}

pub async fn remove_player(username: &str) {
    remove_identity(username).await;

    let rooms = &mut *ROOMS.lock().await;

    for (index, room) in rooms.iter_mut().enumerate() {
        if room.players.remove(username).is_some() {
            if room.players.is_empty() && room.rid != "161" {
                rooms.remove(index);
            }
            return;
        }
    }
}

async fn remove_identity(username: &str) {
    let mut players = PLAYERS.lock().await;

    if let Some((id, _)) = players.iter().find(|x| x.1 == username) {
        let id = id.to_owned();

        players.remove(&id);
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

    for (username, identity) in &*IDENTITIES.lock().await {
        if identity == &identification.identity {
            remove_identity(username).await;
            return Ok(username.to_string());
        }
    }

    Err(())
}

macro_rules! events {
    ($( [$id: expr, $name: expr, $dat: expr] ),*) => {
        {
            #[allow(unused_mut)] // this is a feature

            let mut tmp = Vec::new();
            $(
                if let Ok(res) = Event::new($id, $name, $dat) {
                    tmp.push(res);
                }
            )*
            tmp
        }
    };
}

pub async fn game() {
    let _socket = Socket::new("0.0.0.0:7878", |event| async move {
        info!("{:?}", event);

        if event.name == EventName::Identify {
            return match identify(event.dat).await {
                Ok(username) => {
                    PLAYERS.lock().await.insert(event.id, username);
                    events![[0, EventName::ClearExisted, ()]]
                }
                _ => events![[event.id, EventName::Close, ()]],
            };
        }

        let username = match PLAYERS.lock().await.get(&event.id) {
            None => return events![[event.id, EventName::Close, ()]],
            Some(username) => username,
        }
        .to_string();

        match event.name {
            EventName::Close => {
                // 关闭连接
                remove_player(&username).await;
                events![]
            }
            EventName::WorldMessage | EventName::RoomMessage
                if event.dat.to_string().chars().count() <= 161 =>
            {
                events![[
                    // 聊天信息打包后转发
                    0,
                    event.name,
                    json!( {"sender": username, "message": event.dat})
                ]]
            }
            _ => events![],
        }
    })
    .await;
}
