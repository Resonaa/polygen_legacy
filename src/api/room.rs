use crate::{
    error,
    game::{
        core::ROOMS,
        room::{Room, RoomMap, RoomMode},
    },
    session::UserGuard,
    success, Response,
};
use rocket::serde::{
    json::{json, Json},
    Deserialize,
};

#[get("/room")]
async fn room_list() -> Response {
    success!(*ROOMS.lock().await)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateRoom {
    rid: String,
    mode: RoomMode,
    map: RoomMap,
}

#[post("/room", data = "<create_room>")]
async fn create_room(_user: UserGuard, create_room: Json<CreateRoom>) -> Response {
    let create_room = create_room.into_inner();

    let len = create_room.rid.chars().count();
    if !(1..=12).contains(&len) {
        return error!("房间名称长度不得超过 12 位");
    }

    for room in &*ROOMS.lock().await {
        if room.rid == create_room.rid {
            return error!("房间已存在");
        }
    }

    (*ROOMS.lock().await).push(Room::create(
        &create_room.rid,
        create_room.mode,
        create_room.map,
    ));

    success!("创建成功")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![room_list, create_room]
}
