use rocket::{http::Status, Catcher, Request};
use rocket_dyn_templates::{context, Template};

#[catch(default)]
fn default(status: Status, _req: &Request) -> Template {
    Template::render(
        "error.min",
        context! {
            status: status.to_string()
        },
    )
}

pub fn catchers() -> Vec<Catcher> {
    catchers![default]
}
