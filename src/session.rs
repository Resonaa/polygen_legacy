use crate::{zip_options, concat_vec};
use rocket::{
    http::CookieJar,
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
}

#[derive(Debug)]
pub struct UserGuard(i32, String);

#[rocket::async_trait]
impl<'r> FromRequest<'r> for UserGuard {
    type Error = std::convert::Infallible;

    async fn from_request(request: &'r Request<'_>) -> request::Outcome<UserGuard, Self::Error> {
        fn get<T>(jar: &CookieJar, x: &str) -> Option<T>
        where
            T: std::str::FromStr,
        {
            jar.get_private(x)
                .and_then(|cookie| cookie.value().parse::<T>().ok())
        }

        let jar = request.cookies();
        zip_options!(UserGuard, get(jar, "uid"), get(jar, "username")).or_forward(())
    }
}

#[get("/")]
fn index(user: UserGuard) -> Template {
    Template::render(
        "index",
        context! {
            uid: user.0,
            username: user.1,
        },
    )
}

#[get("/", rank = 2)]
fn no_auth_index() -> Redirect {
    Redirect::to(uri!("/login"))
}

pub fn routes() -> Vec<rocket::Route> {
    concat_vec![routes![index, no_auth_index], login::routes(), register::routes()]
}
