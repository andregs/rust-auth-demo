use rocket::fairing::AdHoc;
use rocket::figment::providers::{Env, Format, Toml};
use rocket::figment::{Figment, Profile};
use rocket::{catchers, Build, Rocket};

use crate::config::*;
use crate::*;

pub async fn build_rocket() -> Rocket<Build> {
    let provider = config_provider();
    rocket::custom(provider)
        .attach(AdHoc::config::<AppConfig>())
        .attach(db::stage().await)
        .attach(redis::stage().await)
        .attach(tracer::stage())
        .attach(controller::stage())
        .register("/", catchers![default_catcher])
}

/// see https://rocket.rs/v0.5-rc/guide/configuration/#custom-providers
fn config_provider() -> Figment {
    Figment::from(rocket::Config::default())
        .merge(Toml::file("App.toml").nested())
        .merge(Env::prefixed("APP_").global())
        .select(Profile::from_env_or("APP_PROFILE", "default"))
}

#[cfg(test)]
pub fn test_config() -> AppConfig {
    config_provider().select(Profile::new("test")).extract().unwrap()
}
