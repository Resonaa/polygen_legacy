use crate::{db::Db, session::UserGuard, success, DbError, Response};
use rocket::serde::json::json;
use rocket_db_pools::Connection;

#[get("/user/info?<username>")]
async fn info(mut db: Connection<Db>, _user: UserGuard, username: String) -> Response {
    let dat = sqlx::query!("SELECT uid FROM user WHERE username = ?", username)
        .fetch_one(&mut *db)
        .await
        .my_conv("未找到该用户")?;

    success!(json!({"uid": dat.uid}))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![info]
}
