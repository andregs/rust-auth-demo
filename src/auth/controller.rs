use redis::Client;
use rocket::fairing::AdHoc;
use rocket::serde::json::Json;
use rocket::{post, routes, State};

use super::*;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Controller", |rocket| async {
        rocket.mount("/", routes![register, login, authenticate])
    })
}

#[post("/register", format = "json", data = "<body>")]
async fn register(body: Json<Credentials>, db: &State<Connection>, redis: &State<Client>) -> String {
    let service = AuthService::new(db, redis);
    let result: bool = service.register(body.0).await;
    
    // TODO 204 no content
    String::from(format!("Registered. {}", result))
}

#[post("/login", format = "json", data = "<body>")]
async fn login(body: Json<Credentials>, db: &State<Connection>, redis: &State<Client>) -> Option<Token> {
    let service = AuthService::new(db, redis);
    // TODO 401 for bad credentials instead of 404
    service.login(body.0).await
}

#[post("/authenticate", format = "text", data = "<body>")]
async fn authenticate(body: Token, db: &State<Connection>, redis: &State<Client>) -> Option<String> {
    let service = AuthService::new(db, redis);
    service.authenticate(body).await
}

// TODO http tests
