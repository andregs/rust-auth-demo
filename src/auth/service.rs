use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::*;

#[async_trait]
pub trait AuthServiceApi {
    async fn register(&self, credentials: Credentials) -> bool;
}

pub struct AuthService<CR = PostgresCredentialRepo>
    where CR: CredentialRepoApi,
{
    db: Pool<Postgres>,
    pub credential_repo: CR,
}

impl AuthService {
    pub fn new(db: &Pool<Postgres>) -> Self {
        Self {
            db: db.clone(),
            credential_repo: PostgresCredentialRepo,
        }
    }
}

#[async_trait]
impl <CR> AuthServiceApi for AuthService<CR>
    where CR: CredentialRepoApi + Sync + Send {

    async fn register(self: &Self, credentials: Credentials) -> bool {
        let mut tx = self.db.begin().await.unwrap();
        let result = self.credential_repo.insert_credentials_tx(&mut tx, &credentials).await;
        tx.commit().await.unwrap();
        result
    }
}
