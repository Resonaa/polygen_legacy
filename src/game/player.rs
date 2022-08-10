use rocket::serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, Default, Hash)]
#[serde(crate = "rocket::serde")]
pub struct Player {
    pub color: i32,
}

impl Player {
    pub fn new(color: i32) -> Self {
        Self { color }
    }
}
