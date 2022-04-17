use anyhow::anyhow;
use rocket::{catch, Request, response::status::Custom, serde::json::Json, http::Status};
use uuid::Uuid;
use std::borrow::Cow;

use super::*;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Duplicated username.")]
    Duplicated(#[source] sqlx::Error),

    #[error("Username is too big.")]
    TooBig(#[source] sqlx::Error),

    #[error("Username and/or password mismatch.")]
    BadCredentials,
    
    #[error("Token does not represent an authenticated user.")]
    BadToken,
    
    #[error("Username must be alphanumeric and it must start with a letter.")]
    BadUsername,
    
    #[error("Username must be between {0} and {1} characters.")]
    BadUsernameSize(usize, usize),
    
    #[error("Password must contain at least {0} characters.")]
    BadPasswordSize(usize),

    #[error("Sorry, we failed.")]
    Other(#[from] anyhow::Error),
}

impl From<sqlx::Error> for Error {
    fn from(source: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref err) = source {
            // https://www.postgresql.org/docs/current/errcodes-appendix.html
            if err.code() == Some(Cow::from("23505")) {
                return Error::Duplicated(source);
            } else if err.code() == Some(Cow::from("22001")) {
                return Error::TooBig(source);
            }
        }

        Error::Other(anyhow!(source))
    }
}

#[derive(Serialize, Debug, Clone)]
pub struct HttpError {
    pub id: String,
    pub msg: String,
}

pub fn to_json<E>(error: &E) -> Json<HttpError> // TODO impl From?
where E: std::error::Error
{
    let id = Uuid::new_v4().to_string();
    let msg = error.to_string();
    
    // TODO proper logging https://github.com/SergioBenitez/Rocket/issues/21
    eprintln!("App Error: {:?}", error);
    Json(HttpError { id, msg })
}

impl From<Error> for Custom<Json<HttpError>> {
    fn from(error: Error) -> Self {
        use Error::*;
        let status = match error {
            Duplicated(_) | TooBig(_) | BadUsername | BadUsernameSize(_,_) | BadPasswordSize(_) => Status::BadRequest,
            BadCredentials | BadToken => Status::Unauthorized,
            Other(_) => Status::InternalServerError,
        };
        let json = to_json(&error);
        Custom(status, json)
    }
}

#[catch(default)]
pub fn default_catcher<'r>(status: Status, req: &'r Request<'_>) -> Custom<Json<HttpError>> {
    let reason = status.reason().unwrap_or("Unknown error.");
    let default = anyhow!(reason);
    let id = Uuid::new_v4().to_string();
    let json = Json(HttpError { id: id.clone(), msg: default.to_string() });
    let cached = req.local_cache(|| json).to_owned();

    if cached.0.id == id { // no cached error was found
        // TODO proper logging https://github.com/SergioBenitez/Rocket/issues/21
        eprintln!("Default Catcher: {:?}", default);
    }
    Custom(status, cached)
}
