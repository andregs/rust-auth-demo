use ::redis::Client;
use rocket::fairing::AdHoc;

use super::*;

pub async fn stage() -> AdHoc {
    AdHoc::on_ignite("Connect to Redis", |rocket| async {
        let config = rocket.state::<AppConfig>().expect("Missing app configuration.");
        let client = open(&config.redis.url);
        rocket.manage(client)
    })
}

pub fn open(redis_url: &str) -> Client {
    Client::open(redis_url).expect("Redis URL check has failed.")
}
