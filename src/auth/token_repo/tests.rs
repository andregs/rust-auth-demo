use uuid::Uuid;

use super::*;
use crate::config::*;

#[async_std::test]
async fn test_save_and_check() {
    let repo = before_each();
    let token = Uuid::new_v4().to_string();
    let username = "username".to_string();
    
    match repo.get_username(&token).await {
        Err(Error::BadToken) => (/* good */),
        _ => panic!("bad token error was expected"),
    }
    
    repo.save_token(&token, &username).await.unwrap();
    let actual = repo.get_username(&token).await
        .expect("username was expected");

    assert_eq!(actual, username);
}

// aux ----

fn before_each() -> RedisTokenRepo {
    let cfg = app::test_config();
    let client = redis::open(&cfg.redis.url);
    let repo = RedisTokenRepo::new(&client);
    repo
}
