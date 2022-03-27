use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Toml};
use rocket::figment::{Figment, Profile};
use rocket::{Build, Rocket};

use super::*;
use crate::auth;

pub async fn build_rocket() -> Rocket<Build> {
    let provider = config_provider();
    rocket::custom(provider)
        .attach(AdHoc::config::<AppConfig>())
        .attach(db::stage().await)
        .attach(redis::stage().await)
        .attach(auth::controller::stage())
}

/// see https://rocket.rs/v0.5-rc/guide/configuration/#custom-providers
fn config_provider() -> Figment {
    Figment::from(rocket::Config::default())
        .merge(Toml::file("App.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"))
}

pub fn extract_config(profile: &str) -> AppConfig {
    config_provider()
        .select(Profile::new(profile))
        .extract()
        .unwrap()
}
