use serde::Deserialize;

pub mod app;
pub mod db;
pub mod redis;

#[derive(Debug, Deserialize, PartialEq)]
pub struct AppConfig {
    pub db: DBConfig,
    pub redis: RedisConfig,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct DBConfig {
    pub url: String,
}

#[derive(Debug, Deserialize, PartialEq)]
pub struct RedisConfig {
    pub url: String,
}
