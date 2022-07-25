use super::{player::Player, map::Map};

pub struct Room {
    pub players: Vec<Player>,
    pub map: Map,
}