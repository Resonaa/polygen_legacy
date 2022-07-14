#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

pub mod api;
pub mod db;
pub mod error;
pub mod game;
pub mod post;
mod responds;
pub mod session;
mod socket;

use regex::Regex;
use rocket::serde::json::{json, Value};

pub trait DbError<T> {
    fn conv(self) -> Result<T, Value>
    where
        Self: std::marker::Sized,
    {
        self.my_conv("数据库错误")
    }

    fn my_conv(self, value: &str) -> Result<T, Value>;
}

#[macro_export]
macro_rules! concat_vec {
    [$($x: expr),*] => {
        {
            let mut tmp = Vec::new();

            $(
                tmp.append(&mut $x);
            )*

            tmp
        }
    };
}

#[macro_export]
macro_rules! message {
    ($event: expr, $msg: expr) => {
        json!({"event": $event, "msg": $msg})
    };
}

impl<T, U> DbError<T> for Result<T, U> {
    fn my_conv(self, value: &str) -> Result<T, Value> {
        self.map_err(|_| json!({"status": "error", "msg": value}))
    }
}

impl<T> DbError<T> for Option<T> {
    fn my_conv(self, value: &str) -> Result<T, Value> {
        self.ok_or_else(|| json!({"status": "error", "msg": value}))
    }
}

pub fn is_valid_username(s: &str) -> bool {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"^[\u4e00-\u9fa5_a-zA-Z0-9]{3,16}$").unwrap();
    }

    RE.is_match(s)
}
