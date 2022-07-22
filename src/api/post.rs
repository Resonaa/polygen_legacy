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
struct Post {
    pid: i64,
    author: String,
    time: String,
    content: String,
    parent: i64,
}

#[get("/post?<parent>&<page>")]
async fn list(
    mut db: Connection<Db>,
    _user: UserGuard,
    parent: i64,
    page: usize,
) -> Result<Value, Value> {
    if !(1..usize::max_value() / 10).contains(&page) {
        return error!("数据范围错误");
    }

    let mut ans = sqlx::query_as!(Post, "SELECT * FROM post WHERE parent = ?", parent)
        .fetch_all(&mut *db)
        .await
        .conv()?;

    let mut q: Vec<_> = ans.iter().map(|x| x.pid).collect();

    while !q.is_empty() {
        let front = q.pop().conv()?;

        let mut res = sqlx::query_as!(Post, "SELECT * FROM post WHERE parent = ?", front)
            .fetch_all(&mut *db)
            .await
            .conv()?;

        q.append(&mut res.iter().map(|x| x.pid).collect());

        ans.append(&mut res);
    }

    ans.sort_unstable_by_key(|x| -x.pid);

    success!(&ans[ans.len().min((page - 1) * 10)..ans.len().min(page * 10)])
}

#[get("/post?<pid>", rank = 2)]
async fn get(mut db: Connection<Db>, _user: UserGuard, pid: i64) -> Result<Value, Value> {
    success!(
        sqlx::query_as!(Post, "SELECT * FROM post WHERE pid = ?", pid)
            .fetch_one(&mut *db)
            .await
            .conv()?
    )
}

#[derive(Deserialize)]
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
    let content = create_post.content.trim();

    if !(1..=100000).contains(&content.chars().count()) {
        return error!("说说长度应为 1~100000 个字符");
    }

    let author = jar.get_private("username").conv()?.value().to_string();

    let time = Local::now().format("%F %T").to_string();

    sqlx::query!(
        "INSERT INTO post (author, time, content, parent) VALUES (?1, ?2, ?3, ?4)",
        author,
        time,
        content,
        create_post.parent
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
) -> Result<Value, Value> {
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
    let mut q = vec![pid];

    let mut cnt = 0;

    while !q.is_empty() {
        let front = q.pop().conv()?;

        let mut res: Vec<_> = sqlx::query!("SELECT pid FROM post WHERE parent = ?", front)
            .fetch_all(&mut *db)
            .await
            .conv()?
            .iter()
            .map(|x| x.pid)
            .collect();

        cnt += res.len();

        q.append(&mut res);
    }

    success!(cnt)
}

pub fn routes() -> Vec<rocket::Route> {
    routes![list, get, create, update, delete, comment_amount]
}
