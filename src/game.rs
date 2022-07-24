#![allow(unused)]

mod core;
mod generator;
mod map;
mod socket;

pub use self::core::game;
use crate::session::{Login, UserGuard};
use crate::{db::Db, error, success, DbError};
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
    serde::json::{json, Json, Value},
};
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn game_page(user: UserGuard) -> Template {
    Template::render("game.min", context! { username: user.username, game: true })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![game_page]
}
