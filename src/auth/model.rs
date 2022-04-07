use anyhow::anyhow;
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
    Duplicated(#[source] sqlx::Error),

    #[error("Username is too big.")]
    TooBig(#[source] sqlx::Error),

    #[error("Username and/or password mismatch.")]
    BadCredentials,
    
    #[error("Token does not represent an authenticated user.")]
    BadToken,

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
