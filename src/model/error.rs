use anyhow::anyhow;
use rocket::{catch, Request, http::Status};
use rocket::response::{status::Custom, Responder};
use rocket::serde::json::{self, Json};
use uuid::Uuid;
use std::borrow::Cow;
use std::fmt::{Debug, Display};

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
    
    #[error("The JSON input is invalid. Details: {0}")]
    BadJson(String),

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

#[derive(Responder, Debug)]
pub struct HttpError {
    body: Custom<Json<ErrorBody>>,
}

#[derive(Serialize, Debug, Clone)]
struct ErrorBody {
    id: String,
    msg: String,
}

impl HttpError {
    pub fn new<E>(status: Status, reason: E) -> Self
    where E: Debug + Display
    {
        let id = Uuid::new_v4().to_string(); // TODO fahring for request ID (correlation)
        let msg = reason.to_string();
        let json = Json(ErrorBody { id, msg });
        let body = Custom(status, json);
        
        // TODO proper logging https://github.com/SergioBenitez/Rocket/issues/21
        eprintln!("App Error: {:?}, Reason: {:?}", body, reason);
        HttpError { body }
    }

    pub fn status(&self) -> Status { self.body.0 }
}

impl From<Error> for HttpError {
    fn from(reason: Error) -> Self {
        use Error::*;

        let status = match reason {
            Duplicated(_) | TooBig(_) | BadUsername | BadUsernameSize(_,_) | BadPasswordSize(_) | BadJson(_) => Status::BadRequest,
            BadCredentials | BadToken => Status::Unauthorized,
            Other(_) => Status::InternalServerError,
        };
        
        HttpError::new(status, reason)
    }
}

impl From<json::Error<'_>> for HttpError {
    fn from(source: json::Error) -> Self {
        let message = source.to_string();
        Error::BadJson(message).into()
    }
}

#[catch(default)]
pub fn default_catcher<'r>(status: Status, _: &'r Request<'_>) -> HttpError {
    let reason = status.reason().unwrap_or("Unknown error.");
    let reason = anyhow!(reason);
    HttpError::new(status, reason)
}
