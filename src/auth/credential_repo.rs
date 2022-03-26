use async_trait::async_trait;
use sqlx::{Pool, Postgres};

use super::*;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CredentialRepoApi {
    async fn insert_credentials(&self, credentials: &Credentials) -> bool;
    async fn check_credentials(&self, credentials: &Credentials) -> bool;
}

pub struct PostgresCredentialRepo {
    db: Pool<Postgres>,
}

impl PostgresCredentialRepo {
    pub fn new(db: &Pool<Postgres>) -> Self {
        Self { db: db.clone() }
    }
}

#[async_trait]
impl CredentialRepoApi for PostgresCredentialRepo {
    async fn insert_credentials(&self, credentials: &Credentials) -> bool {
        // .env file contains a DB url that sqlx macros use on compile-time to validate these queries
        let query = sqlx::query!(
            r#"INSERT INTO credentials (username, password)
            VALUES ($1, crypt($2, gen_salt('bf')))"#,
            credentials.username,
            credentials.password,
        );
        let result = query.execute(&self.db).await.unwrap().rows_affected();
        result == 1
    }

    async fn check_credentials(&self, credentials: &Credentials) -> bool {
        let result = sqlx::query_scalar!(
            // column name is special sqlx syntax to override the inferred type, check query! macro docs
            r#"select password = crypt($1, password) as "not_null!"
            from credentials 
            where username = $2"#,
            credentials.password,
            credentials.username,
        )
        .fetch_optional(&self.db)
        .await
        .unwrap();

        if let Some(is_valid) = result {
            return is_valid;
        }

        false
    }
}

#[cfg(test)]
mod tests;
