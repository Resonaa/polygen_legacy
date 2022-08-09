#![allow(unused)]

pub mod core;
mod player;
mod room;
mod socket;

pub use self::core::game;
use self::core::ROOMS;
use crate::{session::UserGuard, success};
use rocket::{
    http::CookieJar,
    serde::{
        json::{json, Json, Value},
        Deserialize, Serialize,
    },
};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn game_page(user: UserGuard) -> Template {
    Template::render("game.min", context! { username: user.username, game: true })
}

#[get("/list")]
async fn room_list(user: UserGuard) -> Result<Value, Value> {
    success!(*ROOMS.lock().await)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![game_page, room_list]
}
