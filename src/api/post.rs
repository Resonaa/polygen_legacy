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
struct Post<'r> {
    pid: i64,
    author: &'r str,
    time: &'r str,
    content: String,
}

impl<'r> Post<'r> {
    pub fn new(pid: i64, author: &'r str, time: &'r str, content: String) -> Self {
        Self {
            pid,
            author,
            time,
            content,
        }
    }
}

#[get("/post?<page>")]
async fn list(mut db: Connection<Db>, _user: UserGuard, page: i64) -> Result<Value, Value> {
    let page = (page - 1) * 10;

    let dat = sqlx::query!(
        "SELECT * FROM post ORDER BY pid DESC LIMIT 10 OFFSET ?",
        page
    )
    .fetch_all(&mut *db)
    .await
    .conv()?;

    let res: Vec<_> = dat
        .iter()
        .map(|x| Post::new(x.pid, &x.author, &x.time, x.content.to_string()))
        .collect();

    success!(res)
}

#[get("/post?<pid>", rank = 2)]
async fn get(mut db: Connection<Db>, _user: UserGuard, pid: i64) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT * FROM post WHERE pid = ?", pid)
        .fetch_one(&mut *db)
        .await
        .conv()?;

    success!(Post::new(
        dat.pid,
        &dat.author,
        &dat.time,
        dat.content.to_string()
    ))
}

#[post("/post", data = "<post>")]
async fn create(
    mut db: Connection<Db>,
    _user: UserGuard,
    jar: &CookieJar<'_>,
    post: Json<String>,
) -> Result<Value, Value> {
    let s = post.into_inner();

    if !(1..=100000).contains(&s.chars().count()) {
        return error!("说说长度应为 1~100000 个字符");
    }

    let author = jar.get_private("username").conv()?.value().to_string();

    let time = Local::now().format("%F %T").to_string();

    sqlx::query!(
        "INSERT INTO post (author, time, content) VALUES (?1, ?2, ?3)",
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
struct UpdatePost {
    pid: i64,
    content: String,
}

#[put("/post", data = "<update_post>")]
async fn update(
    mut db: Connection<Db>,
    _user: UserGuard,
    update_post: Json<UpdatePost>,
) -> Result<Value, Value> {
    sqlx::query!(
        "UPDATE post SET content = ?1 WHERE pid = ?2",
        update_post.content,
        update_post.pid
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("更新成功")
}

#[delete("/post", data = "<pid>")]
async fn delete(mut db: Connection<Db>, _user: UserGuard, pid: Json<i32>) -> Result<Value, Value> {
    let pid = pid.into_inner();

    sqlx::query!("DELETE FROM post WHERE pid = ?", pid)
        .execute(&mut *db)
        .await
        .conv()?;

    success!("删除成功")
}

#[get("/post/commentamount?<pid>")]
async fn comment_amount(
    mut db: Connection<Db>,
    _user: UserGuard,
    pid: i64,
) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT cid FROM comment WHERE pid = ?", pid)
        .fetch_all(&mut *db)
        .await
        .conv()?
        .len();

    success!(dat)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list, get, create, update, delete, comment_amount]
}
