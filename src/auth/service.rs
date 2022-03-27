use async_trait::async_trait;
use redis::Client;
use sqlx::{Pool, Postgres};
use uuid::Uuid;

use super::*;

#[async_trait]
pub trait AuthServiceApi {
    async fn register(&self, credentials: Credentials) -> bool;
    async fn login(&self, credentials: Credentials) -> Option<Token>;
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
    async fn register(&self, credentials: Credentials) -> bool {
        let mut tx = self.db.begin().await.unwrap();
        let rows_affected = self
            .credential_repo
            .insert_credentials_tx(&mut tx, &credentials)
            .await;

        let result = match rows_affected {
            1 => tx.commit().await.map(|_| true),
            _ => tx.rollback().await.map(|_| false),
        };

        result.unwrap()
    }

    async fn login(&self, credentials: Credentials) -> Option<Token> {
        let valid_credentials = self
            .credential_repo
            .check_credentials_db(&self.db, &credentials)
            .await
            .unwrap_or(false);

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
}

#[cfg(test)]
mod tests;
