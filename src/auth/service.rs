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
    pub credential_repo: CR,
}

impl AuthService {
    pub fn new(db: &Pool<Postgres>) -> Self {
        Self {
            credential_repo: PostgresCredentialRepo::new(db),
        }
    }
}

#[async_trait]
impl <CR> AuthServiceApi for AuthService<CR>
    where CR: CredentialRepoApi + Sync + Send {

    async fn register(self: &Self, credentials: Credentials) -> bool {
        self.credential_repo.insert_credentials(credentials).await
    }
}
