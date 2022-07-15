#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use polygen::{api, db, error, game, post, session};
use rocket::{fs::FileServer, tokio};
use rocket_dyn_templates::Template;
use std::fs;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    tokio::spawn(game::game());

    rocket::build()
        .attach(Template::custom(|engines| {
            for name in ["nav", "prelude", "renderer", "editor"] {
                engines
                    .handlebars
                    .register_partial(
                        name,
                        fs::read_to_string(format!("templates/partials/{name}.min.html.hbs"))
                            .unwrap(),
                    )
                    .unwrap()
            }
        }))
        .attach(db::stage())
        .mount("/", FileServer::from("public/"))
        .mount("/", session::routes())
        .mount("/", post::routes())
        .mount("/api", api::routes())
        .register("/", error::catchers())
}
