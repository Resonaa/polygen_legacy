use super::{core::map::Map, player::Player};
use rocket::serde::{Deserialize, Serialize};
use std::{collections::HashMap, iter::repeat_with};

#[derive(Serialize, Deserialize, Debug, Copy, Clone)]
#[serde(crate = "rocket::serde")]
pub enum RoomStatus {
    Waiting,
    Ongoing,
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
    pub rid: String,
    pub status: RoomStatus,
}

impl Room {
    pub fn new() -> Self {
        Self {
            rid: repeat_with(fastrand::alphanumeric).take(5).collect(),
            ..Default::default()
        }
    }
}
