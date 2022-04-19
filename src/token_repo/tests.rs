use rocket::async_test;
use uuid::Uuid;

use super::*;
use crate::config::*;

#[async_test]
async fn test_save_and_check() {
    let repo = before_each();
    let token = Uuid::new_v4().to_string();
    let username = "username".to_string();

    match repo.get_username(&token).await {
        Err(Error::BadToken) => (/* good */),
        Err(e) => panic!("unexpected error: {:?}", e),
        Ok(username) => panic!("unexpected username: {}", username),
    }

    repo.save_token(&token, &username).await.unwrap();
    let actual = repo.get_username(&token).await.expect("username was expected");

    assert_eq!(actual, username);
}

// aux ----

fn before_each() -> RedisTokenRepo {
    let cfg = app::test_config();
    let client = redis::open(&cfg.redis.url);
    RedisTokenRepo::new(&client)
}
