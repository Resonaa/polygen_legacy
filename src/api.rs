mod announcement;
mod captcha;
mod post;
mod user;

pub fn routes() -> Vec<rocket::Route> {
    [
        announcement::routes(),
        post::routes(),
        user::routes(),
        captcha::routes(),
    ]
    .concat()
}
