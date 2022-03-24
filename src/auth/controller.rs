use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{post, routes};

use super::*;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Controller", |rocket| async {
        rocket.mount("/", routes![register, login, authenticate])
    })
}

#[post("/register", format = "json", data = "<body>")]
async fn register(body: Json<Credentials>) -> String {
    String::from(format!("Registered. {}", body.0.username))
}

#[post("/login", format = "json", data = "<body>")]
async fn login(body: Json<Credentials>) -> String {
    String::from(format!("Login done. {}", body.0.username))
}

#[post("/authenticate", format = "text", data = "<body>")]
async fn authenticate(body: Token) -> String {
    String::from(format!("Authenticated. {}", body))
}
