use super::{Login, UserGuard};
use crate::{db::Db, success, DbError};
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
    serde::json::{json, Json, Value},
};
use rocket_db_pools::{sqlx, Connection};
use rocket_dyn_templates::Template;

#[get("/login")]
fn login(_user: UserGuard) -> Redirect {
    Redirect::to(uri!("/"))
}

#[get("/login", rank = 2)]
fn login_page() -> Template {
    Template::render("login", ())
}

#[post("/login", data = "<login>")]
async fn post_login(
    mut db: Connection<Db>,
    jar: &CookieJar<'_>,
    login: Json<Login<'_>>,
) -> Result<Value, Value> {
    let password = sha256::digest(login.password);

    let uid = sqlx::query!(
        "SELECT uid FROM user WHERE username = ?1 AND password = ?2",
        login.username,
        password
    )
    .fetch_one(&mut *db)
    .await
    .my_conv("用户名或密码错误")?.uid;

    jar.add_private(Cookie::new("username", login.username.to_string()));
    jar.add_private(Cookie::new("uid", uid.to_string()));

    success!("登录成功")
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::named("username"));
    Redirect::to(uri!("/login"))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, login_page, post_login, logout,]
}
