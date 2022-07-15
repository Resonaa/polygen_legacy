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
use std::collections::VecDeque;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct Post<'r> {
    pid: i64,
    author: &'r str,
    time: &'r str,
    content: String,
    parent: i64,
}

impl<'r> Post<'r> {
    pub fn new(pid: i64, author: &'r str, time: &'r str, content: String, parent: i64) -> Self {
        Self {
            pid,
            author,
            time,
            content,
            parent,
        }
    }
}

#[get("/post?<parent>&<page>")]
async fn list(
    mut db: Connection<Db>,
    _user: UserGuard,
    parent: i64,
    page: i64,
) -> Result<Value, Value> {
    let page = (page - 1) * 10;

    let dat = sqlx::query!(
        "SELECT * FROM post WHERE parent = ?1 ORDER BY pid DESC LIMIT 10 OFFSET ?2",
        parent,
        page
    )
    .fetch_all(&mut *db)
    .await
    .conv()?;

    let res: Vec<_> = dat
        .iter()
        .map(|x| Post::new(x.pid, &x.author, &x.time, x.content.to_string(), x.parent))
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
        dat.content.to_string(),
        dat.parent
    ))
}

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
struct CreatePost {
    content: String,
    parent: i64,
}
#[post("/post", data = "<create_post>")]
async fn create(
    mut db: Connection<Db>,
    _user: UserGuard,
    jar: &CookieJar<'_>,
    create_post: Json<CreatePost>,
) -> Result<Value, Value> {
    let s = &create_post.content;

    if !(1..=100000).contains(&s.chars().count()) {
        return error!("说说长度应为 1~100000 个字符");
    }

    let author = jar.get_private("username").conv()?.value().to_string();

    let time = Local::now().format("%F %T").to_string();

    sqlx::query!(
        "INSERT INTO post (author, time, content, parent) VALUES (?1, ?2, ?3, ?4)",
        author,
        time,
        s,
        create_post.parent
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
    let mut q = VecDeque::new();
    q.push_back(pid);

    let mut cnt = 0;

    while !q.is_empty() {
        let front = q.front().conv()?.to_owned();
        q.pop_front();

        let dat = sqlx::query!("SELECT pid FROM post WHERE parent = ?", front)
            .fetch_all(&mut *db)
            .await
            .conv()?;

        let mut res: VecDeque<_> = dat.iter().map(|x| x.pid).collect();
        cnt += res.len();

        q.append(&mut res);
    }

    success!(cnt)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list, get, create, update, delete, comment_amount]
}
