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
    let new_id: i64 = service.register(body.0).await.unwrap();
    
    // TODO 204 no content
    String::from(format!("Registered. {}", new_id))
}

#[post("/login", format = "json", data = "<body>")]
async fn login(body: Json<Credentials>, db: &State<Connection>, redis: &State<Client>) -> Token {
    let service = AuthService::new(db, redis);
    // TODO 401 for bad credentials instead of 404
    service.login(body.0).await.unwrap()
}

#[post("/authenticate", format = "text", data = "<body>")]
async fn authenticate(body: Token, db: &State<Connection>, redis: &State<Client>) -> String {
    let service = AuthService::new(db, redis);
    service.authenticate(body).await.unwrap()
}

// TODO http tests
// TODO implement error handling
// TODO publish to k8s
// TODO health check
// TODO graceful shutdown
// TODO consume external http service (correlate requests)
// TODO improve logging
// TODO externalize more config attributes like pool size
