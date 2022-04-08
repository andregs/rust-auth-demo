use anyhow::anyhow;
use rocket::serde::{Deserialize, Serialize};
use std::borrow::Cow;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

pub type Token = String;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct LoginOk {
    pub token: Token,
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct AuthOk {
    pub username: String,
}

pub type Result<T> = core::result::Result<T, Error>;

pub mod error;
use error::*;
