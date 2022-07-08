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
    Template::render("register.min", ())
}

#[post("/register", data = "<register>")]
async fn post_register(
    mut db: Connection<Db>,
    jar: &CookieJar<'_>,
    register: Json<Login<'_>>,
) -> Result<Value, Value> {
    if register.captcha.to_lowercase()
        != jar
            .get_private("captcha")
            .conv()?
            .value()
            .to_string()
            .to_lowercase()
    {
        return error!("验证码错误");
    }

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

    jar.add_private(Cookie::new("username", register.username.to_string()));

    success!("注册成功")
}

pub fn routes() -> Vec<rocket::Route> {
    routes![register, register_page, post_register]
}
