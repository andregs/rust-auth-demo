use redis::Client;
use rocket::fairing::AdHoc;
use rocket::{http::Status, post, routes, State};
use rocket::response::status::{Custom, Created};
use rocket::serde::json::Json;
use serde::Serialize;
use uuid::Uuid;

use super::*;

pub fn stage() -> AdHoc {
    AdHoc::on_ignite("Auth Controller", |rocket| async {
        rocket.mount("/", routes![register, login, authenticate])
    })
}

#[post("/register", format = "json", data = "<body>")]
async fn register(body: Json<Credentials>, db: &State<Connection>, redis: &State<Client>) -> RestResult<Created<&'static str>> {
    let service = AuthService::new(db, redis);
    let new_id: i64 = service.register(body.0).await?;
    let location = format!("/profile/{}", new_id); // TODO rocket::uri!
    Ok(Created::new(location))
}

#[post("/login", format = "json", data = "<body>")]
async fn login(body: Json<Credentials>, db: &State<Connection>, redis: &State<Client>) -> RestResult<Token> {
    let service = AuthService::new(db, redis);
    let token = service.login(body.0).await?;
    Ok(token) // TODO json body
}

#[post("/authenticate", format = "text", data = "<body>")]
async fn authenticate(body: Token, db: &State<Connection>, redis: &State<Client>) -> RestResult<String> {
    let service = AuthService::new(db, redis);
    let username = service.authenticate(body).await?;
    Ok(username) // TODO json body
}

type RestResult<T> = core::result::Result<T, Custom<Json<RestError>>>;

#[derive(Serialize)]
struct RestError {
    id: String,
    msg: String,
}

// TODO maybe there's no need to build the json body here, just http status and raw error as payload,
// then I can create individual rocket error catchers to convert from http status to json response
impl From<Error> for Custom<Json<RestError>> {
    fn from(error: Error) -> Self {
        let id = Uuid::new_v4().to_string();
        let msg = error.to_string();
        let body = Json(RestError { id, msg });
        match error {
            Error::Duplicated(_) | Error::TooBig(_) => Custom(Status::BadRequest, body),
            Error::BadCredentials | Error::BadToken => Custom(Status::Unauthorized, body),
            Error::Other(source) => {
                // TODO proper logging
                eprintln!("Oops... {:?}", source);
                Custom(Status::InternalServerError, body)
            },
        }
    }
}

// TODO http tests
// TODO implement error handling
// TODO publish to k8s
// TODO health check
// TODO graceful shutdown
// TODO consume external http service (correlate requests)
// TODO improve logging
// TODO externalize more config attributes like pool size
