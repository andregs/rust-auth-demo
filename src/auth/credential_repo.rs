use async_trait::async_trait;
use sqlx::{Executor, Pool, Postgres};
use std::borrow::Cow;

use super::*;

pub type Connection = Pool<Postgres>;
pub type Transaction = sqlx::Transaction<'static, Postgres>;
type Result<T> = core::result::Result<T, Error>;

#[cfg_attr(test, mockall::automock)]
#[async_trait]
pub trait CredentialRepoApi {
    async fn insert_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Result<u64>;
    async fn check_credentials_db(&self, db: &Connection, credentials: &Credentials) -> Result<bool>;
    async fn check_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Result<bool>;
}

pub struct PostgresCredentialRepo;

#[async_trait]
impl CredentialRepoApi for PostgresCredentialRepo {
    async fn insert_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Result<u64> {
        self.insert_credentials(tx, credentials).await
    }

    async fn check_credentials_db(&self, db: &Connection, credentials: &Credentials) -> Result<bool> {
        self.check_credentials(db, credentials).await
    }

    async fn check_credentials_tx(&self, tx: &mut Transaction, credentials: &Credentials) -> Result<bool> {
        self.check_credentials(tx, credentials).await
    }
}

impl PostgresCredentialRepo {
    async fn insert_credentials<'ex, EX>(&self, executor: EX, credentials: &Credentials) -> Result<u64>
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
        .map(|result| result.rows_affected())
        .map_err(|err| err.into())
    }
    
    async fn check_credentials<'ex, EX>(&self, executor: EX, credentials: &Credentials) -> Result<bool>
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
        .map(|option| option.or(Some(false)).unwrap())
        .map_err(|err| err.into())
    }
}

#[derive(thiserror::Error, Debug)]
pub enum Error {
    #[error("Duplicated username.")]
    Duplicated,

    #[error("Username is too big.")]
    TooBig,
    
    #[error("What!?")]
    Unknown,
}

impl From<sqlx::Error> for Error {
    fn from(err: sqlx::Error) -> Self {
        if let sqlx::Error::Database(ref err) = err {
            // https://www.postgresql.org/docs/current/errcodes-appendix.html
            if err.code() == Some(Cow::from("23505")) {
                return Error::Duplicated;
            } else if err.code() == Some(Cow::from("22001")) {
                return Error::TooBig;
            }
        }

        // TODO proper log the backtrace
        eprintln!("Unknown {:?}", err);
        Error::Unknown
    }
}

#[cfg(test)]
mod tests;
