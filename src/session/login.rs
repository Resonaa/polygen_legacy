use super::{Login, UserGuard};
use crate::{db::Db, error, success, DbError, Response};
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
    serde::json::{json, Json},
};
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::{context, Template};

#[get("/login")]
fn login(_user: UserGuard) -> Redirect {
    Redirect::to(uri!("/"))
}

#[get("/login", rank = 2)]
fn login_page() -> Template {
    Template::render("session.min", context! { session: true, login: true })
}

#[post("/login", data = "<login>")]
async fn post_login(mut db: Connection<Db>, jar: &CookieJar<'_>, login: Json<Login>) -> Response {
    if login.captcha.to_lowercase() != jar.get_private("captcha").conv()?.value().to_lowercase() {
        return error!("验证码错误");
    }

    let password = sha256::digest(&login.password);

    sqlx::query!(
        "SELECT uid FROM user WHERE username = ?1 AND password = ?2",
        login.username,
        password
    )
    .fetch_one(&mut *db)
    .await
    .my_conv("用户名或密码错误")?;

    jar.add_private(Cookie::new("username", login.username.clone()));

    success!("登录成功")
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::named("username"));
    Redirect::to(uri!("/login"))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, login_page, post_login, logout]
}
