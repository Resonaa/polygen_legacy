use rocket::serde::json::{json, Json, Value};
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
};
use rocket_dyn_templates::Template;

use super::{Login, User};
use crate::db::Db;
use crate::{error, success, DbError};

#[get("/login")]
fn login(_user: User) -> Redirect {
    Redirect::to(uri!("/"))
}

#[get("/login", rank = 2)]
fn login_page() -> Template {
    Template::render("login", ())
}

#[post("/login", data = "<login>")]
fn post_login(jar: &CookieJar<'_>, login: Json<Login<'_>>) -> Result<Value, Value> {
    let db = Db::new().conv()?;
    let mut stmt = db
        .prepare(&format!(
            "SELECT uid, password FROM user WHERE username = '{}'",
            login.username
        ))
        .conv()?;
    let dat = stmt
        .query_map([], |row| {
            Ok((row.get::<usize, i32>(0)?, row.get::<usize, String>(1)?))
        })
        .conv()?
        .next()
        .ok_or_else(|| error!("用户名或密码错误"))?
        .unwrap();
    if login.password == dat.1 {
        jar.add_private(Cookie::new("username", login.username.to_string()));
        jar.add_private(Cookie::new("uid", dat.0.to_string()));
        Ok(success!("登录成功"))
    } else {
        Err(error!("用户名或密码错误"))
    }
}

#[post("/logout")]
fn logout(jar: &CookieJar<'_>) -> Redirect {
    jar.remove_private(Cookie::named("username"));
    Redirect::to(uri!("/login"))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![login, login_page, post_login, logout,]
}
