use rocket::serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub type Token = String;

pub type Result<T> = core::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Duplicated username.")]
    Duplicated,

    #[error("Username is too big.")]
    TooBig,

    #[error("Username and/or password mismatch.")]
    BadCredentials,
    
    #[error("Token does not represent an authenticated user.")]
    BadToken,

    #[error("Sorry, we failed.")]
    Unknown,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref err) = err {
            // https://www.postgresql.org/docs/current/errcodes-appendix.html
            if err.code() == Some(Cow::from("23505")) {
                return Error::Duplicated;
            } else if err.code() == Some(Cow::from("22001")) {
                return Error::TooBig;
            }
        }

        // TODO proper log the backtrace
        eprintln!("Unknown {:?}", err);
        Error::Unknown
    }
}

impl From<redis::RedisError> for Error {
    fn from(err: redis::RedisError) -> Self {
        // TODO proper log the backtrace
        eprintln!("Unknown {:?}", err);
        Error::Unknown
    }
}
