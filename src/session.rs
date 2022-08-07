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
pub struct Login {
    username: String,
    password: String,
    captcha: String,
}

#[derive(Debug)]
pub struct UserGuard {
    pub username: String,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserGuard, Self::Error> {
        request
            .cookies()
            .get_private("username")
            .map(|cookie| UserGuard {
                username: cookie.value().to_string(),
            })
            .or_forward(())
    }
}

#[get("/")]
fn index(user: UserGuard) -> Template {
    Template::render(
        "index.min",
        context! {
            username: user.username,
            home: true,
        },
    )
}

#[get("/", rank = 2)]
fn no_auth_index() -> Redirect {
    Redirect::to(uri!("/login"))
}

pub fn routes() -> Vec<rocket::Route> {
    [
        routes![index, no_auth_index],
        login::routes(),
        register::routes(),
    ]
    .concat()
}
