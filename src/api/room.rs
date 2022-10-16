use crate::{game::core::ROOMS, success, Response};
use rocket::serde::json::json;

#[get("/room")]
async fn room_list() -> Response {
    success!(*ROOMS.lock().await)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![room_list]
}
