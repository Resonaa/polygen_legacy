use crate::session::UserGuard;
use rocket_dyn_templates::{context, Template};

#[get("/<_>")]
fn get_post(user: UserGuard) -> Template {
    Template::render(
        "index.min",
        context! {
            username: user.username,
        },
    )
}

pub fn routes() -> Vec<rocket::Route> {
    routes![get_post]
}
