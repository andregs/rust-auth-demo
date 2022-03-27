use uuid::Uuid;

use super::*;
use crate::config::*;

#[async_std::test]
async fn test_save_and_check() {
    let repo = before_each();
    let token = Uuid::new_v4().to_string();
    let username = "username".to_string();
    
    assert!(repo.get_username(&token).await.is_none());
    
    repo.save_token(&token, &username).await;
    let actual = repo.get_username(&token).await;

    assert!(actual.is_some());
    assert_eq!(actual.unwrap(), username);
}

// aux ----

fn before_each() -> RedisTokenRepo {
    let cfg = app::extract_config("test");
    let client = redis::open(&cfg.redis.url);
    let repo = RedisTokenRepo::new(&client);
    repo
}
