use anyhow::anyhow;
use rocket::{async_trait, catch, Request, http::Status, outcome::Outcome};
use rocket::response::{self, status::Custom, Responder, Response};
use rocket::serde::json::{self, Json};
use std::borrow::Cow;
use std::fmt::{Debug, Display};

use crate::tracer::Tracer;

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

#[derive(Debug)]
pub struct HttpError {
    status: Status,
    display_message: String,
    debug_message: String,
}

impl HttpError {
    pub fn new<E>(status: Status, reason: E) -> Self
    where E: Debug + Display
    {
        HttpError {
            status,
            display_message: format!("{}", reason),
            debug_message: format!("{:?}", reason),
        }
    }
}

#[async_trait]
impl<'r> Responder<'r, 'static> for HttpError {
    fn respond_to(self, req: &'r Request<'_>) -> response::Result<'static> {
        let log = req.local_cache(|| Tracer::new());
        log.error(&self.debug_message);
        let payload = JsonPayload {
            id: &log.request_id(),
            msg: &self.display_message,
        };
        let custom = Custom(self.status, Json(payload));
        Response::build_from(custom.respond_to(req)?).ok()
    }
}

#[derive(Serialize, Debug)]
struct JsonPayload<'r> {
    id: &'r str,
    msg: &'r str,
}

#[catch(default)]
pub fn default_catcher<'r>(status: Status, _: &'r Request<'_>) -> HttpError {
    let reason = status.reason().unwrap_or("Unknown error.");
    let reason = anyhow!(reason);
    HttpError::new(status, reason)
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

impl From<json::Error<'_>> for HttpError {
    fn from(source: json::Error) -> Self {
        let message = source.to_string();
        Error::BadJson(message).into()
    }
}

impl<'r, S, F> From<Error> for Outcome<S, (Status, HttpError), F> {
    fn from(error: Error) -> Self {
        let error: HttpError = error.into();
        let status = error.status;
        Outcome::Failure((status, error))
    }
}
