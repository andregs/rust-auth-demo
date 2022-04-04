use uuid::Uuid;

use super::*;
use crate::config::*;

mod insert_credentials {
    use super::*;

    #[async_std::test]
    async fn it_should_insert_good_credentials() {
        let (mut tx, repo) = before_each().await;
        let credentials = &new_random_credentials();
        let rows_affected = repo.insert_credentials(&mut tx, credentials).await.unwrap();
        assert_eq!(rows_affected, 1);
    }

    #[async_std::test]
    async fn it_should_reject_duplicated_username() {
        let (mut tx, repo) = before_each().await;
        let credentials = &new_random_credentials();
        let rows_affected = repo.insert_credentials(&mut tx, credentials).await.unwrap();
        assert_eq!(rows_affected, 1);

        let result = repo.insert_credentials(&mut tx, credentials).await;
        match result {
            // TODO one day we'll have assert_matches https://github.com/rust-lang/rust/issues/82775
            Err(Error::Duplicated) => (),
            bad => panic!("Unexpected {:?}", bad),
        }
    }

    #[async_std::test]
    async fn it_should_reject_username_too_big() {
        let (mut tx, repo) = before_each().await;
        let mut credentials = new_random_credentials();
        credentials.username = format!("{0}{0}", credentials.username);

        let result = repo.insert_credentials(&mut tx, &credentials).await;
        match result {
            Err(Error::TooBig) => (),
            bad => panic!("Unexpected {:?}", bad),
        }
    }

    // TODO test of MVCC, where a tx tries to read data that has already been changed by another concurrent tx
}

mod check_credentials {
    use super::*;

    #[async_std::test]
    async fn it_should_find_valid_credentials() {
        let (mut tx, repo) = before_each().await;
        let credentials = &new_random_credentials();
        repo.insert_credentials(&mut tx, credentials).await.unwrap();
        
        let is_valid = repo.check_credentials_tx(&mut tx, credentials).await.unwrap();
        assert!(is_valid);
    }

    #[async_std::test]
    async fn it_should_not_find_when_username_is_wrong() {
        let (mut tx, repo) = before_each().await;
        let credentials = new_random_credentials();
        
        let is_valid = repo.check_credentials_tx(&mut tx, &credentials).await.unwrap();
        assert!(is_valid == false);
    }

    #[async_std::test]
    async fn it_should_not_find_when_password_is_wrong() {
        let (mut tx, repo) = before_each().await;
        let mut credentials = new_random_credentials();
        repo.insert_credentials(&mut tx, &credentials).await.unwrap();
        credentials.password = String::from("wrong password");
        
        let is_valid = repo.check_credentials_tx(&mut tx, &credentials).await.unwrap();
        assert!(is_valid == false);
    }
}

// aux ----

async fn before_each() -> (Transaction, PostgresCredentialRepo) {
    let db = connect().await;
    let tx = db.begin().await.unwrap();
    let repo = PostgresCredentialRepo;
    (tx, repo)
}

async fn connect() -> Pool<Postgres> {
    let cfg = app::test_config();
    db::connect(&cfg.db.url).await
}

fn new_random_credentials() -> Credentials {
    let uuid = Uuid::new_v4().to_string();
    let username = format!("test-{}", uuid);
    let password = uuid;    
    Credentials{ username, password }
}
