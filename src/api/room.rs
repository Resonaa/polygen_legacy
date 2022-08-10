use crate::{
    error,
    game::{
        core::{remove_player, IDENTITIES, ROOMS},
        player::Player,
    },
    session::UserGuard,
    success, Response,
};
use rocket::serde::json::{json, Json};
use std::iter::repeat_with;

#[get("/room")]
async fn room_list(_user: UserGuard) -> Response {
    success!(*ROOMS.lock().await)
}

#[post("/room", data = "<rid>")]
async fn join_room(user: UserGuard, rid: Json<usize>) -> Response {
    let rid = rid.into_inner();

    remove_player(&user.username).await;

    for room in &mut *ROOMS.lock().await {
        if room.rid == rid {
            room.players.insert(user.username.clone(), Player::new(0));

            let identity: String = repeat_with(fastrand::alphanumeric).take(10).collect();
            IDENTITIES
                .lock()
                .await
                .insert(user.username, identity.clone());

            return success!(identity);
        }
    }

    error!("加入失败")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![room_list, join_room]
}
