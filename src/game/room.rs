use super::player::Player;
use rocket::serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum RoomMode {
    Hexagon,
    Quadrilateral,
}

impl Default for RoomMode {
    fn default() -> Self {
        Self::Hexagon
    }
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub enum RoomMap {
    Random,
}

impl Default for RoomMap {
    fn default() -> Self {
        Self::Random
    }
}

#[derive(Default, Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Room {
    pub players: HashMap<String, Player>,
    pub rid: String,
    pub ongoing: bool,
    pub mode: RoomMode,
    pub map: RoomMap,
}

impl Room {
    pub fn new(rid: &str) -> Self {
        Self {
            rid: rid.to_string(),
            ..Default::default()
        }
    }

    pub fn create(rid: &str, mode: RoomMode, map: RoomMap) -> Self {
        Self {
            rid: rid.to_string(),
            mode,
            map,
            ..Default::default()
        }
    }
}
