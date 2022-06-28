use super::{Login, User};
use crate::{db::Db, error, success, DbError};
use regex::Regex;
use rocket::{
    http::{Cookie, CookieJar},
    response::Redirect,
    serde::json::{json, Json, Value},
};
use rocket_dyn_templates::Template;
use rusqlite::params;

#[get("/register")]
fn register(_user: User) -> Redirect {
    Redirect::to(uri!("/"))
}

#[get("/register", rank = 2)]
fn register_page() -> Template {
    Template::render("register", ())
}

#[post("/register", data = "<register>")]
fn post_register(jar: &CookieJar<'_>, register: Json<Login<'_>>) -> Result<Value, Value> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\u4e00-\u9fa5_a-zA-Z0-9]+$").unwrap();
    }

    if !RE.is_match(register.username) {
        return Err(error!("用户名只允许包含中文、英文、数字和_"));
    }
    let len = register.username.chars().count();
    if !(3..=16).contains(&len) {
        return Err(error!("用户名长度应为 3 ~ 16 位"));
    }
    let len = register.password.chars().count();
    if !(6..=20).contains(&len) {
        return Err(error!("密码长度应为 6 ~ 20 位"));
    }

    let db = Db::new().conv()?;
    let mut stmt = db
        .prepare(&format!(
            "SELECT uid FROM user WHERE username = '{}'",
            register.username
        ))
        .conv()?;
    if stmt
        .query_map([], |row| row.get::<usize, i32>(0))
        .conv()?
        .count()
        > 0
    {
        return Err(error!("用户名已存在"));
    }

    db.execute(
        "INSERT INTO user (username, password, exp) VALUES (?1, ?2, 0)",
        params![register.username, register.password],
    )
    .conv()?;

    let mut stmt = db.prepare("SELECT uid FROM user").conv()?;
    let dat = stmt
        .query_map([], |row| row.get::<usize, i32>(0))
        .conv()?
        .last()
        .ok_or_else(|| error!("用户名或密码错误"))?
        .unwrap();

    jar.add_private(Cookie::new("username", register.username.to_string()));
    jar.add_private(Cookie::new("uid", dat.to_string()));

    Ok(success!("注册成功"))
}

pub fn routes() -> Vec<rocket::Route> {
    routes![register, register_page, post_register]
}
