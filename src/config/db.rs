use rocket::fairing::AdHoc;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use super::*;

pub async fn stage() -> AdHoc {
    AdHoc::on_ignite("Connect to DB", |rocket| async {
        let config = rocket.state::<AppConfig>().expect("Missing app configuration.");
        let db = connect(&config.db.url).await;
        rocket.manage(db)
    })
}

pub async fn connect(database_url: &String) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await
        .expect("Unable to connect")
}
