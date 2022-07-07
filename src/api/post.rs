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

#[get("/post?<page>")]
async fn list(mut db: Connection<Db>, _user: UserGuard, page: i32) -> Result<Value, Value> {
    let page = (page - 1) * 10;

    let dat = sqlx::query!(
        "SELECT author, time, content FROM post ORDER BY pid DESC LIMIT 10 OFFSET ?",
        page
    )
    .fetch_all(&mut *db)
    .await
    .conv()?;

    let res: Vec<(i64, &String, &String)> = dat
        .iter()
        .map(|x| (x.author, &x.time, &x.content))
        .collect();

    success!(res)
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

    let author = jar
        .get_private("uid")
        .conv()?
        .value()
        .parse::<i32>()
        .conv()?;

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
struct UpdatePost<'r> {
    pid: i32,
    content: &'r str,
}

#[put("/post", data = "<update_post>")]
async fn update(
    mut db: Connection<Db>,
    _user: UserGuard,
    update_post: Json<UpdatePost<'_>>,
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

pub fn routes() -> Vec<rocket::Route> {
    return routes![list, create, update, delete];
}
