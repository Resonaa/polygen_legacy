mod announcement;
mod captcha;
mod comment;
mod post;
mod user;

use crate::concat_vec;

pub fn routes() -> Vec<rocket::Route> {
    concat_vec![
        announcement::routes(),
        post::routes(),
        user::routes(),
        captcha::routes(),
        comment::routes()
    ]
}
