use redis::Client;
use rocket::fairing::AdHoc;
use rocket::{http::Status, post, routes, State};
use rocket::response::status::{Custom, Created};
use rocket::serde::{json::Json, Serialize};
use uuid::Uuid;

use super::*;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Controller", |rocket| async {
        rocket.mount("/", routes![register, login, authenticate])
    })
}

// TODO input validation

#[post("/register", format = "json", data = "<body>")]
async fn register(body: Json<Credentials>, db: &State<Connection>, redis: &State<Client>) -> RestResult<Created<&'static str>> {
    let service = AuthService::new(db, redis);
    let new_id: i64 = service.register(body.0).await?;
    let location = format!("/profile/{}", new_id); // TODO rocket::uri!
    let body = Created::new(location);
    Ok(body)
}

#[post("/login", format = "json", data = "<body>")]
async fn login(body: Json<Credentials>, db: &State<Connection>, redis: &State<Client>) -> RestResult<Json<LoginOk>> {
    let service = AuthService::new(db, redis);
    let token = service.login(body.0).await?;
    let body = Json(LoginOk{ token });
    Ok(body)
}

#[post("/authenticate", format = "text", data = "<body>")]
async fn authenticate(body: Token, db: &State<Connection>, redis: &State<Client>) -> RestResult<Json<AuthOk>> {
    let service = AuthService::new(db, redis);
    let username = service.authenticate(body).await?;
    let body = Json(AuthOk{ username });
    Ok(body)
}

type RestResult<T> = core::result::Result<T, Custom<Json<RestError>>>;

#[derive(Serialize)]
struct RestError {
    id: String,
    msg: String,
}

mod error;

// TODO http tests
// TODO publish to k8s
// TODO health check
// TODO graceful shutdown
// TODO consume external http service (correlate requests)
// TODO improve logging
// TODO externalize more config attributes like pool size
