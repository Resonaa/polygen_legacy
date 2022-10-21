use rocket::serde::{
    json::{self, serde_json, Value},
    Deserialize, Serialize,
};

#[derive(Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[serde(crate = "rocket::serde")]
pub enum EventName {
    Close,
    Abort,
    Message,
    Identify,
    WorldMessage,
    RoomMessage,
    ClearExisted,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
#[serde(crate = "rocket::serde")]
pub struct Event {
    pub id: i32,
    pub name: EventName,
    pub dat: Value,
}

impl Event {
    pub fn new(
        id: i32,
        name: EventName,
        dat: impl Serialize,
    ) -> Result<Self, serde_json::error::Error> {
        Ok(Self {
            id,
            name,
            dat: json::to_value(dat)?,
        })
    }

    pub fn from(id: i32, value: &str) -> Result<Self, serde_json::error::Error> {
        #[derive(Serialize, Deserialize)]
        #[serde(crate = "rocket::serde")]
        struct FromEvent {
            name: EventName,
            dat: Value,
        }

        let value: FromEvent = json::from_str(value)?;

        Ok(Self {
            id,
            name: value.name,
            dat: value.dat,
        })
    }
}
