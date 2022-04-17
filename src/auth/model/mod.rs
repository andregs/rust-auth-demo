use rocket::response::status::Custom;
use rocket::serde::{json::Json, Deserialize, Serialize};

pub type Token = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct LoginOk {
    pub token: Token,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct AuthOk {
    pub username: String,
}

mod credentials;
pub use credentials::*;

mod error;
pub use error::*;

pub type HttpResult<T> = core::result::Result<T, Custom<Json<HttpError>>>;
pub type Result<T> = core::result::Result<T, Error>;
