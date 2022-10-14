mod announcement;
mod captcha;
mod comment;
mod post;
mod room;
mod user;

pub fn routes() -> Vec<rocket::Route> {
    [
        announcement::routes(),
        post::routes(),
        user::routes(),
        captcha::routes(),
        room::routes(),
        comment::routes(),
    ]
    .concat()
}
