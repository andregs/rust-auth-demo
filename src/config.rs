use serde::Deserialize;

pub mod app;
pub mod db;

#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub db: DBConfig,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DBConfig {
    pub url: String,
}
