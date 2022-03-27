use uuid::Uuid;

use super::*;
use crate::config::*;

mod insert_credentials {
    use super::*;

    #[async_std::test]
    async fn it_should_insert_good_credentials() {
        let (mut tx, repo) = before_each().await;
        let credentials = &new_random_credentials();
        let rows_affected = repo.insert_credentials(&mut tx, credentials).await;
        assert_eq!(rows_affected, 1);
    }
}

mod check_credentials {
    use super::*;

    #[async_std::test]
    async fn it_should_find_valid_credentials() {
        let (mut tx, repo) = before_each().await;
        let credentials = &new_random_credentials();
        repo.insert_credentials(&mut tx, credentials).await;
        
        let is_valid = repo.check_credentials_tx(&mut tx, credentials).await;
        assert!(is_valid.unwrap());
    }

    #[async_std::test]
    async fn it_should_not_find_when_username_is_wrong() {
        let (mut tx, repo) = before_each().await;
        let credentials = new_random_credentials();
        
        let is_valid = repo.check_credentials_tx(&mut tx, &credentials).await;
        assert!(is_valid.is_none());
    }

    #[async_std::test]
    async fn it_should_not_find_when_password_is_wrong() {
        let (mut tx, repo) = before_each().await;
        let mut credentials = new_random_credentials();
        repo.insert_credentials(&mut tx, &credentials).await;
        credentials.password = String::from("wrong password");
        
        let is_valid = repo.check_credentials_tx(&mut tx, &credentials).await;
        assert!(is_valid.unwrap() == false);
    }
}

// aux ----

async fn before_each() -> (Transaction, PostgresCredentialRepo) {
    let cfg = app::extract_config("test");
    let db = db::connect(&cfg.db.url).await;
    let tx = db.begin().await.unwrap();
    let repo = PostgresCredentialRepo;
    (tx, repo)
}

fn new_random_credentials() -> Credentials {
    let uuid = Uuid::new_v4().to_string();
    let username = format!("test-{}", uuid);
    let password = uuid;    
    Credentials{ username, password }
}
