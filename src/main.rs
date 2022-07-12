#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use polygen::{api, db, error, post, session};
use rocket::fs::FileServer;
use rocket_dyn_templates::Template;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    rocket::build()
        .attach(Template::fairing())
        .attach(db::stage())
        .mount("/", FileServer::from("public/"))
        .mount("/", session::routes())
        .mount("/", post::routes())
        .mount("/api", api::routes())
        .register("/", error::catchers())
}
