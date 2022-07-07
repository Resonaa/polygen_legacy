use crate::{db::Db, session::UserGuard, success, DbError};
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize, Serialize,
};
use rocket_db_pools::Connection;

#[get("/announcement")]
async fn list(mut db: Connection<Db>, _user: UserGuard) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT title, content FROM announcement")
        .fetch_all(&mut *db)
        .await
        .conv()?;

    let res: Vec<(&String, &String)> = dat
        .iter()
        .map(|x| (&x.title, &x.content))
        .collect();

    success!(res)
}

#[get("/announcement/<aid>")]
async fn read(mut db: Connection<Db>, _user: UserGuard, aid: i32) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT title, content FROM announcement WHERE aid = ?", aid)
        .fetch_one(&mut *db)
        .await
        .conv()?;

    success!((dat.title, dat.content))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Announcement<'r> {
    title: &'r str,
    content: &'r str,
}

#[post("/announcement", data = "<announcement>")]
async fn create(
    mut db: Connection<Db>,
    _user: UserGuard,
    announcement: Json<Announcement<'_>>,
) -> Result<Value, Value> {
    sqlx::query!(
        "INSERT INTO announcement (title, content) VALUES (?1, ?2)",
        announcement.title,
        announcement.content
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("发布成功")
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UpdateAnnouncement<'r> {
    aid: i32,
    title: &'r str,
    content: &'r str,
}

#[put("/announcement", data = "<update_announcement>")]
async fn update(
    mut db: Connection<Db>,
    _user: UserGuard,
    update_announcement: Json<UpdateAnnouncement<'_>>,
) -> Result<Value, Value> {
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
async fn delete(mut db: Connection<Db>, _user: UserGuard, aid: Json<i32>) -> Result<Value, Value> {
    let aid = aid.into_inner();
    sqlx::query!("DELETE FROM announcement WHERE aid = ?", aid)
        .execute(&mut *db)
        .await
        .conv()?;

    success!("删除成功")
}

pub fn routes() -> Vec<rocket::Route> {
    return routes![list, read, create, update, delete];
}
