use crate::{db::Db, session::UserGuard, success, DbError};
use rocket::serde::{json::{json, Value}};
use rocket_db_pools::Connection;

#[get("/user/info?<uid>")]
async fn info(mut db: Connection<Db>, _user: UserGuard, uid: i32) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT username FROM user WHERE uid = ?", uid)
        .fetch_one(&mut *db)
        .await
        .conv()?;

    success!(json!({"username": dat.username}))
}

pub fn routes() -> Vec<rocket::Route> {
    return routes![info];
}
