use crate::session::UserGuard;
use rocket_dyn_templates::{context, Template};

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
fn no_auth_index() -> Template {
    Template::render(
        "index.min",
        context! {
            home: true
        },
    )
}

#[get("/post/<_>")]
fn post_view(user: UserGuard) -> Template {
    Template::render(
        "post_view.min",
        context! {
            username: user.username
        },
    )
}

#[get("/post/<_>", rank = 2)]
fn no_auth_post_view() -> Template {
    Template::render("post_view.min", context! {})
}

pub fn routes() -> Vec<rocket::Route> {
    routes![index, no_auth_index, post_view, no_auth_post_view]
}
