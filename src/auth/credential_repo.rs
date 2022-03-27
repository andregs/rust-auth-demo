use async_trait::async_trait;
use sqlx::{Executor, Pool, Postgres};

use super::*;

pub type Connection = Pool<Postgres>;
pub type Transaction = sqlx::Transaction<'static, Postgres>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CredentialRepoApi {
    async fn insert_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> u64;
    async fn check_credentials_db(&self, db: &Connection, credentials: &Credentials) -> Option<bool>;
    async fn check_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Option<bool>;
}

pub struct PostgresCredentialRepo;

#[async_trait]
impl CredentialRepoApi for PostgresCredentialRepo {
    async fn insert_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> u64 {
        self.insert_credentials(tx, credentials).await
    }

    async fn check_credentials_db(&self, db: &Connection, credentials: &Credentials) -> Option<bool> {
        self.check_credentials(db, credentials).await
    }

    async fn check_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Option<bool> {
        self.check_credentials(tx, credentials).await
    }
}

impl PostgresCredentialRepo {
    async fn insert_credentials<'ex, EX>(&self, executor: EX, credentials: &Credentials) -> u64
    where
        EX: 'ex + Executor<'ex, Database = Postgres>,
    {
        // .env file contains a DB url that sqlx macros use on compile-time to validate these queries
        sqlx::query!(
            r#"INSERT INTO credentials (username, password)
            VALUES ($1, crypt($2, gen_salt('bf')))"#,
            credentials.username,
            credentials.password,
        )
        .execute(executor)
        .await
        .unwrap()
        .rows_affected()
    }
    
    async fn check_credentials<'ex, EX>(&self, executor: EX, credentials: &Credentials) -> Option<bool>
    where
        EX: 'ex + Executor<'ex, Database = Postgres>,
    {
        sqlx::query_scalar!(
            // column name is special sqlx syntax to override the inferred type, check query! macro docs
            r#"select password = crypt($1, password) as "not_null!"
            from credentials 
            where username = $2"#,
            credentials.password,
            credentials.username,
        )
        .fetch_optional(executor)
        .await
        .unwrap()
    }
}

#[cfg(test)]
mod tests;
