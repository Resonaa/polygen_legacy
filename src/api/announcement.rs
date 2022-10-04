use crate::{db::Db, session::UserGuard, success, DbError, Response};
use rocket::serde::{
    json::{json, Json},
    Deserialize, Serialize,
};
use rocket_db_pools::Connection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Announcement {
    aid: i64,
    title: String,
    content: String,
}

#[get("/announcement")]
async fn list(mut db: Connection<Db>, _user: UserGuard) -> Response {
    success!(sqlx::query_as!(Announcement, "SELECT * FROM announcement")
        .fetch_all(&mut *db)
        .await
        .conv()?)
}

#[get("/announcement?<aid>", rank = 2)]
async fn get(mut db: Connection<Db>, _user: UserGuard, aid: i32) -> Response {
    success!(sqlx::query_as!(
        Announcement,
        "SELECT * FROM announcement WHERE aid = ?",
        aid
    )
    .fetch_one(&mut *db)
    .await
    .conv()?)
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateAnnouncement {
    title: String,
    content: String,
}

#[post("/announcement", data = "<create_announcement>")]
async fn create(
    mut db: Connection<Db>,
    _user: UserGuard,
    create_announcement: Json<CreateAnnouncement>,
) -> Response {
    sqlx::query!(
        "INSERT INTO announcement (title, content) VALUES (?1, ?2)",
        create_announcement.title,
        create_announcement.content
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("发布成功")
}

#[put("/announcement", data = "<update_announcement>")]
async fn update(
    mut db: Connection<Db>,
    _user: UserGuard,
    update_announcement: Json<Announcement>,
) -> Response {
    sqlx::query!(
        "UPDATE announcement SET title = ?1, content = ?2 WHERE aid = ?3",
        update_announcement.title,
        update_announcement.content,
        update_announcement.aid
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("更新成功")
}

#[delete("/announcement", data = "<aid>")]
async fn delete(mut db: Connection<Db>, _user: UserGuard, aid: Json<i32>) -> Response {
    let aid = aid.into_inner();
    sqlx::query!("DELETE FROM announcement WHERE aid = ?", aid)
        .execute(&mut *db)
        .await
        .conv()?;

    success!("删除成功")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list, get, create, update, delete]
}
