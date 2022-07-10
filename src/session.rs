use crate::concat_vec;
use rocket::{
    outcome::IntoOutcome,
    request::{self, FromRequest, Request},
    response::Redirect,
    serde::{Deserialize, Serialize},
};
use rocket_dyn_templates::{context, Template};

mod login;
mod register;

#[derive(Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Login<'r> {
    username: &'r str,
    password: &'r str,
    captcha: &'r str,
}

#[derive(Debug)]
pub struct UserGuard(String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserGuard, Self::Error> {
        request
            .cookies()
            .get_private("username")
            .map(|cookie| UserGuard(cookie.value().to_string()))
            .or_forward(())
    }
}

#[get("/")]
fn index(user: UserGuard) -> Template {
    Template::render(
        "index.min",
        context! {
            username: user.0,
        },
    )
}

#[get("/", rank = 2)]
fn no_auth_index() -> Redirect {
    Redirect::to(uri!("/login"))
}

pub fn routes() -> Vec<rocket::Route> {
    concat_vec![
        routes![index, no_auth_index],
        login::routes(),
        register::routes()
    ]
}
