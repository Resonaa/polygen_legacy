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
struct Post {
    pid: i64,
    author: String,
    time: String,
    content: String,
}

#[get("/post?<page>")]
async fn list(mut db: Connection<Db>, page: i32) -> Response {
    let offset = (page - 1) * 10;

    let ans = sqlx::query_as!(
        Post,
        "SELECT * FROM post ORDER BY pid DESC LIMIT 10 OFFSET ?",
        offset
    )
    .fetch_all(&mut *db)
    .await
    .conv()?;

    success!(ans)
}

#[get("/post?<pid>", rank = 2)]
async fn get(mut db: Connection<Db>, pid: i64) -> Response {
    success!(
        sqlx::query_as!(Post, "SELECT * FROM post WHERE pid = ?", pid)
            .fetch_one(&mut *db)
            .await
            .conv()?
    )
}

#[post("/post", data = "<content>")]
async fn create(
    mut db: Connection<Db>,
    _user: UserGuard,
    jar: &CookieJar<'_>,
    content: Json<String>,
) -> Response {
    let content = content.into_inner();

    if !(1..=100000).contains(&content.chars().count()) {
        return error!("说说长度应为 1~100000 个字符");
    }

    let author = jar.get_private("username").conv()?.value().to_string();

    let time = Local::now().format("%F %T").to_string();

    sqlx::query!(
        "INSERT INTO post (author, time, content) VALUES (?1, ?2, ?3)",
        author,
        time,
        content
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("发布成功")
}

#[derive(Deserialize)]
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
) -> Response {
    let content = update_post.content.trim();

    sqlx::query!(
        "UPDATE post SET content = ?1 WHERE pid = ?2",
        content,
        update_post.pid
    )
    .execute(&mut *db)
    .await
    .conv()?;

    success!("更新成功")
}

#[delete("/post", data = "<pid>")]
async fn delete(mut db: Connection<Db>, _user: UserGuard, pid: Json<i32>) -> Response {
    let pid = pid.into_inner();

    sqlx::query!("DELETE FROM post WHERE pid = ?", pid)
        .execute(&mut *db)
        .await
        .conv()?;

    success!("删除成功")
}

// #[get("/post/commentamount?<pid>")]
// async fn comment_amount(mut db: Connection<Db>, _user: UserGuard, pid: i64) -> Response {
//     let mut q = vec![pid];

//     let mut cnt = 0;

//     while !q.is_empty() {
//         let front = q.pop().conv()?;

//         let mut res: Vec<_> = sqlx::query!("SELECT pid FROM post WHERE parent = ?", front)
//             .fetch_all(&mut *db)
//             .await
//             .conv()?
//             .iter()
//             .map(|x| x.pid)
//             .collect();

//         cnt += res.len();

//         q.append(&mut res);
//     }

//     success!(cnt)
// }

pub fn routes() -> Vec<rocket::Route> {
    routes![list, get, create, update, delete]
}
