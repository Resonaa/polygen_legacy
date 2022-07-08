use super::{Login, UserGuard};
use crate::{db::Db, error, is_valid_username, success, DbError};
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
    serde::json::{json, Json, Value},
};
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::Template;

#[get("/register")]
fn register(_user: UserGuard) -> Redirect {
    Redirect::to(uri!("/"))
}

#[get("/register", rank = 2)]
fn register_page() -> Template {
    Template::render("register", ())
}

#[post("/register", data = "<register>")]
async fn post_register(
    mut db: Connection<Db>,
    jar: &CookieJar<'_>,
    register: Json<Login<'_>>,
) -> Result<Value, Value> {
    if !is_valid_username(register.username) {
        return error!("用户名长度应为 3 ~ 16 位，包含中文、英文、数字和_");
    }

    let len = register.password.chars().count();
    if !(6..=20).contains(&len) {
        return error!("密码长度应为 6 ~ 20 位");
    }

    let password = sha256::digest(register.password);
    sqlx::query!(
        "INSERT INTO user (username, password) VALUES (?1, ?2)",
        register.username,
        password
    )
    .execute(&mut *db)
    .await
    .my_conv("用户名已存在")?;

    let uid = sqlx::query!("SELECT uid FROM user WHERE username = ?", register.username)
        .fetch_one(&mut *db)
        .await
        .conv()?
        .uid;

    jar.add_private(Cookie::new("username", register.username.to_string()));
    jar.add_private(Cookie::new("uid", uid.to_string()));

    success!("注册成功")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![register, register_page, post_register]
}
