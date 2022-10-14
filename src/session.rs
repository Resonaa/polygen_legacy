use rocket::{
    outcome::IntoOutcome,
    request::{self, FromRequest, Request},
    serde::{Deserialize, Serialize},
};

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

pub fn routes() -> Vec<rocket::Route> {
    [login::routes(), register::routes()].concat()
}
