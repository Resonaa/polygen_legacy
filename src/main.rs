#[macro_use]
extern crate rocket;

use rocket::fs::FileServer;
use rocket_dyn_templates::Template;
use polygen::session;


#[launch]
async fn rocket() -> _ {
    rocket::build()
        .attach(Template::fairing())
        .mount("/", FileServer::from("public/"))
        .mount("/", session::routes())
        .mount("/", session::login::routes())
        .mount("/", session::register::routes())
}
