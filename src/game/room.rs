use super::{core::map::Map, player::Player};
use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub enum RoomStatus {
    Waiting,
    Ongoing,
    Error,
}

impl Default for RoomStatus {
    fn default() -> Self {
        Self::Waiting
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Room {
    pub players: Vec<Player>,
    pub map: Map,
    pub rid: i32,
    pub status: RoomStatus,
}
