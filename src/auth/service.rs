use async_trait::async_trait;
use redis::Client;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::*;

#[async_trait]
pub trait AuthServiceApi {
    async fn register(&self, credentials: Credentials) -> i64;
    async fn login(&self, credentials: Credentials) -> Option<Token>;
    async fn authenticate(&self, token: Token) -> Option<String>;
}

pub struct AuthService<CR = PostgresCredentialRepo, TR = RedisTokenRepo>
where
    CR: CredentialRepoApi,
    TR: TokenRepoApi,
{
    db: Pool<Postgres>,
    credential_repo: CR,
    token_repo: TR,
}

impl AuthService {
    pub fn new(db: &Pool<Postgres>, redis: &Client) -> Self {
        Self {
            db: db.clone(),
            credential_repo: PostgresCredentialRepo,
            token_repo: RedisTokenRepo::new(redis),
        }
    }
}

#[async_trait]
impl<CR, TR> AuthServiceApi for AuthService<CR, TR>
where
    CR: CredentialRepoApi + Sync + Send,
    TR: TokenRepoApi + Sync + Send,
{
    async fn register(&self, credentials: Credentials) -> i64 {
        let mut tx = self.db.begin().await.unwrap(); // TODO handle error
        let new_id = self
            .credential_repo
            .insert_credentials_tx(&mut tx, &credentials)
            .await;

        let result = match new_id {
            Ok(id) => tx.commit().await,
            Err(_) => tx.rollback().await,
        };

        new_id.unwrap() // TODO handle error
    }

    async fn login(&self, credentials: Credentials) -> Option<Token> {
        let valid_credentials = self
            .credential_repo
            .check_credentials_db(&self.db, &credentials)
            .await
            .unwrap_or(false); // TODO handle error

        if valid_credentials {
            let uuid = Uuid::new_v4().to_string();
            self.token_repo
                .save_token(&uuid, &credentials.username)
                .await;

            Some(uuid)
        } else {
            None
        }
    }

    async fn authenticate(&self, token: Token) -> Option<String> {
        self.token_repo.get_username(&token).await
    }
}

#[cfg(test)]
mod tests;
