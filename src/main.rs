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
            for entry in fs::read_dir("templates/partials/").unwrap() {
                let entry = entry.unwrap();

                let file_name = entry.file_name().into_string().unwrap();

                let name = file_name.trim_end_matches(".min.html.hbs");

                engines
                    .handlebars
                    .register_partial(name, fs::read_to_string(entry.path()).unwrap())
                    .unwrap()
            }
        }))
        .attach(db::stage())
        .mount("/", FileServer::from("public/"))
        .mount("/", session::routes())
        .mount("/post", post::routes())
        .mount("/api", api::routes())
        .mount("/game", game::routes())
        .register("/", error::catchers())
}
