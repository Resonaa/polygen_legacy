use crate::{db::Db, error, session::UserGuard, success, DbError, Response};
use chrono::prelude::*;
use rocket::{
    http::CookieJar,
    serde::{
        json::{json, Json},
        Deserialize, Serialize,
    },
};
use rocket_db_pools::Connection;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Comment {
    cid: i64,
    author: String,
    time: String,
    content: String,
    pid: i64,
}

#[get("/comment?<pid>&<page>")]
async fn list(mut db: Connection<Db>, pid: i64, page: i32) -> Response {
    let offset = (page - 1) * 10;

    let ans = sqlx::query_as!(
        Comment,
        "SELECT * FROM comment WHERE pid = ?1 ORDER BY cid DESC LIMIT 10 OFFSET ?2",
        pid,
        offset
    )
    .fetch_all(&mut *db)
    .await
    .conv()?;

    success!(ans)
}

#[get("/comment?<cid>", rank = 2)]
async fn get(mut db: Connection<Db>, cid: i64) -> Response {
    success!(
        sqlx::query_as!(Comment, "SELECT * FROM comment WHERE cid = ?", cid)
            .fetch_one(&mut *db)
            .await
            .conv()?
    )
}

#[derive(Deserialize)]
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
) -> Response {
    let create_comment = create_comment.into_inner();
    let content = create_comment.content;
    let pid = create_comment.pid;

    if !(1..=100000).contains(&content.chars().count()) {
        return error!("评论长度应为 1~100000 个字符");
    }

    let author = jar.get_private("username").conv()?.value().to_string();

    let time = Local::now().format("%F %T").to_string();

    sqlx::query!(
        "INSERT INTO comment (author, time, content, pid) VALUES (?1, ?2, ?3, ?4)",
        author,
        time,
        content,
        pid
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("发布成功")
}

#[derive(Deserialize)]
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
) -> Response {
    let content = update_comment.content.trim();

    sqlx::query!(
        "UPDATE comment SET content = ?1 WHERE cid = ?2",
        content,
        update_comment.cid
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("更新成功")
}

#[delete("/comment", data = "<cid>")]
async fn delete(mut db: Connection<Db>, _user: UserGuard, cid: Json<i64>) -> Response {
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
