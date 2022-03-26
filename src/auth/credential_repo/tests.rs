use uuid::Uuid;

use super::*;
use crate::config::*;

mod insert_credentials {
    use super::*;

    #[async_std::test]
    async fn it_should_insert_good_credentials() {
        let repo = create_repo().await;
        let credentials = &new_random_credentials();
        let is_inserted = repo.insert_credentials(credentials).await;
        assert!(is_inserted);
    }
}

mod check_credentials {
    use super::*;

    #[async_std::test]
    async fn it_should_find_valid_credentials() {
        let repo = create_repo().await;
        let credentials = &new_random_credentials();
        repo.insert_credentials(credentials).await;
        
        let is_valid = repo.check_credentials(credentials).await;
        assert!(is_valid);
    }

    #[async_std::test]
    async fn it_should_not_find_when_username_is_wrong() {
        let repo = create_repo().await;
        let credentials = new_random_credentials();
        
        let is_valid = repo.check_credentials(&credentials).await;
        assert!(is_valid == false);
    }

    #[async_std::test]
    async fn it_should_not_find_when_password_is_wrong() {
        let repo = create_repo().await;
        let mut credentials = new_random_credentials();
        repo.insert_credentials(&credentials).await;
        credentials.password = String::from("wrong password");
        
        let is_valid = repo.check_credentials(&credentials).await;
        assert!(is_valid == false);
    }
}

// aux ----

async fn create_repo() -> PostgresCredentialRepo {
    let cfg = app::extract_config("test");
    let db = db::connect(&cfg.db.url).await;
    PostgresCredentialRepo::new(&db)
}

fn new_random_credentials() -> Credentials {
    let uuid = Uuid::new_v4().to_string();
    let username = format!("test-{}", uuid);
    let password = uuid;    
    Credentials{ username, password }
}
