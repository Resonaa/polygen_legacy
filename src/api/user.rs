use crate::{db::Db, session::UserGuard, success, DbError};
use rocket::serde::json::{json, Value};
use rocket_db_pools::Connection;

#[get("/user/info?<username>")]
async fn info(mut db: Connection<Db>, _user: UserGuard, username: String) -> Result<Value, Value> {
    let dat = sqlx::query!("SELECT uid FROM user WHERE username = ?", username)
        .fetch_one(&mut *db)
        .await
        .conv()?;

    success!(json!({"uid": dat.uid}))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![info]
}
