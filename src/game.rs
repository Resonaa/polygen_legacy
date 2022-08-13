pub mod core;
mod event;
pub mod player;
mod room;
mod socket;

pub use self::core::game;
use self::{
    core::{remove_player, IDENTITIES, ROOMS},
    player::Player,
};
use crate::session::UserGuard;
use rocket_dyn_templates::{context, Template};
use std::iter::repeat_with;

#[get("/")]
fn room_list(user: UserGuard) -> Template {
    Template::render("game.min", context! { username: user.username, game: true })
}

#[get("/<rid>")]
async fn game_core(user: UserGuard, rid: String) -> Template {
    remove_player(&user.username).await;

    for room in &mut *ROOMS.lock().await {
        if room.rid == rid {
            room.players.insert(user.username.clone(), Player::new(0));

            let identity: String = repeat_with(fastrand::alphanumeric).take(10).collect();
            IDENTITIES
                .lock()
                .await
                .insert(user.username.clone(), identity.clone());

            return Template::render(
                "game_core.min",
                context! { username: user.username, identity: identity },
            );
        }
    }

    Template::render("game.min", context! { username: user.username, game: true })
}

pub fn routes() -> Vec<rocket::Route> {
    routes![room_list, game_core]
}
