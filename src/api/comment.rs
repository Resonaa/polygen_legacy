use crate::{db::Db, error, session::UserGuard, success, DbError};
use chrono::prelude::*;
use rocket::{
    http::CookieJar,
    serde::{
        json::{json, Json, Value},
        Deserialize, Serialize,
    },
};
use rocket_db_pools::Connection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Comment<'r> {
    cid: i64,
    pid: i64,
    author: &'r str,
    time: &'r str,
    content: String,
}

impl<'r> Comment<'r> {
    pub fn new(cid: i64, pid: i64, author: &'r str, time: &'r str, content: String) -> Self {
        Self {
            cid,
            pid,
            author,
            time,
            content,
        }
    }
}

#[get("/comment?<cid>")]
async fn list(mut db: Connection<Db>, _user: UserGuard, cid: i32) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT * FROM comment WHERE cid = ?", cid)
        .fetch_one(&mut *db)
        .await
        .conv()?;

    success!(Comment::new(
        dat.cid,
        dat.pid,
        &dat.author,
        &dat.time,
        dat.content.to_string()
    ))
}

#[get("/comment?<pid>&<page>", rank = 2)]
async fn get(
    mut db: Connection<Db>,
    _user: UserGuard,
    pid: i32,
    page: i32,
) -> Result<Value, Value> {
    let page = (page - 1) * 10;

    let dat = sqlx::query!(
        "SELECT * FROM comment WHERE pid = ?1 ORDER BY cid DESC LIMIT 10 OFFSET ?2",
        pid,
        page
    )
    .fetch_all(&mut *db)
    .await
    .conv()?;

    let res: Vec<_> = dat
        .iter()
        .map(|x| Comment::new(x.cid, x.pid, &x.author, &x.time, x.content.to_string()))
        .collect();

    success!(res)
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreateComment {
    pid: i64,
    content: String,
}

#[post("/comment", data = "<create_comment>")]
async fn create(
    mut db: Connection<Db>,
    _user: UserGuard,
    jar: &CookieJar<'_>,
    create_comment: Json<CreateComment>,
) -> Result<Value, Value> {
    let s = create_comment.content.to_string();

    if !(1..=100000).contains(&s.chars().count()) {
        return error!("评论长度应为 1~100000 个字符");
    }

    sqlx::query!("SELECT pid FROM post WHERE pid = ?", create_comment.pid)
        .fetch_one(&mut *db)
        .await
        .my_conv("说说不存在")?;

    let author = jar.get_private("username").conv()?.value().to_string();

    let time = Local::now().format("%F %T").to_string();

    sqlx::query!(
        "INSERT INTO comment (pid, author, time, content) VALUES (?1, ?2, ?3, ?4)",
        create_comment.pid,
        author,
        time,
        s
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("发布成功")
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct UpdateComment {
    cid: i64,
    content: String,
}

#[put("/comment", data = "<update_comment>")]
async fn update(
    mut db: Connection<Db>,
    _user: UserGuard,
    update_comment: Json<UpdateComment>,
) -> Result<Value, Value> {
    sqlx::query!(
        "UPDATE comment SET content = ?1 WHERE cid = ?2",
        update_comment.content,
        update_comment.cid
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("更新成功")
}

#[delete("/comment", data = "<cid>")]
async fn delete(mut db: Connection<Db>, _user: UserGuard, cid: Json<i64>) -> Result<Value, Value> {
    let cid = cid.into_inner();

    sqlx::query!("DELETE FROM comment WHERE cid = ?", cid)
        .execute(&mut *db)
        .await
        .conv()?;

    success!("删除成功")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list, get, create, update, delete]
}
