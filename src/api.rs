mod announcement;
mod post;
mod user;
mod captcha;

use crate::concat_vec;

pub fn routes() -> Vec<rocket::Route> {
    concat_vec![announcement::routes(), post::routes(), user::routes(), captcha::routes()]
}
