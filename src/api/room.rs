use crate::{game::core::ROOMS, session::UserGuard, success, Response};
use rocket::serde::json::json;

#[get("/room")]
async fn room_list(_user: UserGuard) -> Response {
    success!(*ROOMS.lock().await)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![room_list]
}
