use async_trait::async_trait;
use redis::{AsyncCommands, Client};

use super::*;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait TokenRepoApi {
    async fn save_token(&self, token: &Token, username: &str);
}

pub struct RedisTokenRepo {
    client: Client,
}

impl RedisTokenRepo {
    pub fn new(client: &Client) -> Self {
        Self {
            client: client.clone(),
        }
    }
}

#[async_trait]
impl TokenRepoApi for RedisTokenRepo {
    async fn save_token(&self, token: &Token, username: &str) {
        // redis-rs currently doesn't have connection pooling
        let mut conn = self.client.get_async_connection().await.unwrap();
        let key = format!("token:{}", token);
        let value = username;
        conn.set(key, value).await.unwrap()
    }
}
