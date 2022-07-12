use crate::{db::Db, session::UserGuard, success, DbError};
use rocket::serde::{
    json::{json, Json, Value},
    Deserialize, Serialize,
};
use rocket_db_pools::Connection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Announcement<'r> {
    aid: i64,
    title: &'r str,
    content: String,
}

impl<'r> Announcement<'r> {
    pub fn new(aid: i64, title: &'r str, content: String) -> Self {
        Self {
            aid,
            title,
            content,
        }
    }
}

#[get("/announcement")]
async fn list(mut db: Connection<Db>, _user: UserGuard) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT * FROM announcement")
        .fetch_all(&mut *db)
        .await
        .conv()?;

    let res: Vec<_> = dat
        .iter()
        .map(|x| Announcement::new(x.aid, &x.title, x.content.to_string()))
        .collect();

    success!(res)
}

#[get("/announcement?<aid>", rank = 2)]
async fn get(mut db: Connection<Db>, _user: UserGuard, aid: i32) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT * FROM announcement WHERE aid = ?", aid)
        .fetch_one(&mut *db)
        .await
        .conv()?;

    success!(Announcement::new(
        dat.aid,
        &dat.title,
        dat.content.to_string()
    ))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateAnnouncement<'r> {
    title: &'r str,
    content: &'r str,
}

#[post("/announcement", data = "<create_announcement>")]
async fn create(
    mut db: Connection<Db>,
    _user: UserGuard,
    create_announcement: Json<CreateAnnouncement<'_>>,
) -> Result<Value, Value> {
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
    update_announcement: Json<Announcement<'_>>,
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
    return routes![list, get, create, update, delete];
}
