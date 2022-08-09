use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
#[serde(crate = "rocket::serde")]
pub enum LandType {
    Land,
    Crown,
    City,
    Mountain,
}

impl Default for LandType {
    fn default() -> Self {
        Self::Land
    }
}

#[derive(Serialize, Deserialize, Debug, Default, Clone, Copy)]
#[serde(crate = "rocket::serde")]
pub struct Land {
    pub color: i32,
    pub amount: i32,
    pub tp: LandType,
}

impl Land {
    pub fn new(color: i32, amount: i32, tp: LandType) -> Self {
        Self { color, amount, tp }
    }
}
