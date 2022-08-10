pub mod core;
mod event;
pub mod player;
mod room;
mod socket;

pub use self::core::game;
use crate::session::UserGuard;
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn game_page(user: UserGuard) -> Template {
    Template::render("game.min", context! { username: user.username, game: true })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![game_page]
}
