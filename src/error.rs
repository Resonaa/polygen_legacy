use rocket::{http::Status, response::Redirect, Request};
use rocket_dyn_templates::{context, Template};

#[catch(default)]
pub fn default(status: Status, _req: &Request) -> Template {
    Template::render(
        "error.min",
        context! {
            status: status.to_string()
        },
    )
}

#[catch(404)]
pub fn avatar_not_found() -> Redirect {
    Redirect::to(uri!("/img/defaultAvatar.jpg"))
}
