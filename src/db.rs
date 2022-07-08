use rocket::{
    fairing::{self, AdHoc},
    serde::{Deserialize, Serialize},
    Build, Rocket,
};
use rocket_db_pools::{sqlx, Database};

#[derive(Database)]
#[database("db")]
pub struct Db(sqlx::SqlitePool);

#[derive(Debug, Clone, Deserialize, Serialize)]
#[serde(crate = "rocket::serde")]
pub struct User {
    uid: i32,
    username: String,
    password: String,
}

pub async fn run_migrations(rocket: Rocket<Build>) -> fairing::Result {
    match Db::fetch(&rocket) {
        Some(db) => match sqlx::migrate!("./db/migrations").run(&**db).await {
            Ok(_) => Ok(rocket),
            Err(e) => {
                error!("Failed to initialize SQLx database: {}", e);
                Err(rocket)
            }
        },

        None => Err(rocket),
    }
}

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("SQLx Stage", |rocket| async {
        rocket
            .attach(Db::init())
            .attach(AdHoc::try_on_ignite("SQLx Migrations", run_migrations))
    })
}
