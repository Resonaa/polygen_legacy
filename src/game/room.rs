use super::{core::map::Map, player::Player};
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

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
    pub players: HashMap<String, Player>,
    pub map: Map,
    pub rid: usize,
    pub status: RoomStatus,
}

impl Room {
    pub fn new(rid: usize) -> Self {
        Self {
            rid,
            ..Default::default()
        }
    }
}
