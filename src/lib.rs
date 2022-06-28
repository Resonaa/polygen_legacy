#[macro_use]
extern crate rocket;
#[macro_use]
extern crate lazy_static;

mod db;
mod responds;
pub mod session;
mod user;

use rocket::serde::json::{json, Value};

pub trait DbError<T> {
    fn conv(self) -> Result<T, Value>;
    fn my_conv(self, value: Value) -> Result<T, Value>;
}

#[macro_export]
/// User, Some(1), Some(2) -> User(1, 2)
macro_rules! zip_options {
    ($s: ident, $($x: expr),+) => {
        (|| {
            Some($s($ (
                $x?
            ),+))
        })()
    };
}

impl<T, U> DbError<T> for Result<T, U> {
    fn conv(self) -> Result<T, Value> {
        self.my_conv(error!("数据库错误"))
    }

    fn my_conv(self, value: Value) -> Result<T, Value> {
        self.map_err(|_| value)
    }
}
