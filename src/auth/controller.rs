use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{post, routes, State};
use sqlx::{Pool, Postgres};

use super::*;
use super::service::AuthService;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Controller", |rocket| async {
        rocket.mount("/", routes![register, login, authenticate])
    })
}

#[post("/register", format = "json", data = "<body>")]
async fn register(db: &State<Pool<Postgres>>, body: Json<Credentials>) -> String {
    let service = AuthService::new(db);
    let result: bool = service.register(body.0).await;
    String::from(format!("Registered. {}", result))
}

#[post("/login", format = "json", data = "<body>")]
async fn login(body: Json<Credentials>) -> String {
    String::from(format!("Login done. {}", body.0.username))
}

#[post("/authenticate", format = "text", data = "<body>")]
async fn authenticate(body: Token) -> String {
    String::from(format!("Authenticated. {}", body))
}
