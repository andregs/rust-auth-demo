use redis::Client;
use rocket::{catchers, post, routes, State};
use rocket::fairing::AdHoc;
use rocket::response::status::Created;
use rocket::serde::json::Json;

use super::*;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Controller", |rocket| async {
        rocket
            .mount("/", routes![register, login, authenticate])
            .register("/", catchers![default_catcher])
    })
}

#[post("/register", format = "json", data = "<body>")]
async fn register(body: Credentials, db: &State<Connection>, redis: &State<Client>) -> HttpResult<Created<&'static str>> {
    let service = AuthService::new(db, redis);
    let new_id: i64 = service.register(body).await?;
    
    // TODO create a /profile/<username> route that requires authentication
    let location = format!("/profile/{}", new_id); // TODO use rocket::uri!

    let body = Created::new(location);
    Ok(body)
}

#[post("/login", format = "json", data = "<body>")]
async fn login(body: Credentials, db: &State<Connection>, redis: &State<Client>) -> HttpResult<Json<LoginOk>> {
    let service = AuthService::new(db, redis);
    let token = service.login(body).await?;
    let body = Json(LoginOk{ token });
    Ok(body)
}

#[post("/authenticate", format = "text", data = "<body>")]
async fn authenticate(body: Token, db: &State<Connection>, redis: &State<Client>) -> HttpResult<Json<AuthOk>> {
    let service = AuthService::new(db, redis);
    let username = service.authenticate(body).await?;
    let body = Json(AuthOk{ username });
    Ok(body)
}

// TODO health check
// TODO graceful shutdown
// TODO consume external http service (correlate requests)
// TODO externalize more config attributes (e.g. pool size)
