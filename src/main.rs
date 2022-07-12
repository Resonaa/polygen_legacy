#[macro_use]
extern crate rocket;

use dotenv::dotenv;
use polygen::{api, db, error, game, post, session};
use rocket::{fs::FileServer, tokio};
use rocket_dyn_templates::Template;

#[launch]
async fn rocket() -> _ {
    dotenv().ok();

    env_logger::builder().format_timestamp(None).init();

    tokio::spawn(async move {
        game::start_ws().await;
    });

    rocket::build()
        .attach(Template::fairing())
        .attach(db::stage())
        .mount("/", FileServer::from("public/"))
        .mount("/", session::routes())
        .mount("/", post::routes())
        .mount("/api", api::routes())
        .register("/", error::catchers())
}
