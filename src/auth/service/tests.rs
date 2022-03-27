use uuid::Variant;

use super::*;
use crate::config::*;

mod register {
    use super::*;

    #[async_std::test]
    async fn it_should_return_true_when_exactly_1_user_is_registered() {
        let mut svc = before_each().await;
        svc.credential_repo
            .expect_insert_credentials_tx()
            .once()
            .return_const(1_u64);

        let (username, password) = ("a".into(), "b".into());
        let credentials = Credentials { username, password };
        let actual = svc.register(credentials).await;
        assert_eq!(actual, true);
    }

    #[async_std::test]
    async fn it_should_return_false_when_not_exactly_1_user_is_registered() {
        for case in vec![0, 2] {
            let mut svc = before_each().await;
            svc.credential_repo
                .expect_insert_credentials_tx()
                .once()
                .return_const(case as u64);

            let (username, password) = ("a".into(), "b".into());
            let credentials = Credentials { username, password };
            let actual = svc.register(credentials).await;
            assert_eq!(actual, false);
        }
    }
}

mod login {
    use super::*;

    #[async_std::test]
    async fn it_should_return_uuid_token_when_login_is_ok() {
        let mut svc = before_each().await;
        svc.credential_repo
            .expect_check_credentials_db()
            .once()
            .return_const(true);

        svc.token_repo.expect_save_token().once().return_const(());

        let (username, password) = ("a".into(), "b".into());
        let credentials = Credentials { username, password };

        let actual = svc.login(credentials).await.unwrap();
        let actual = Uuid::parse_str(&actual).unwrap();
        assert_eq!(actual.get_variant(), Variant::RFC4122);
    }

    #[async_std::test]
    async fn it_should_not_return_uuid_token_when_login_fails() {
        let mut svc = before_each().await;
        svc.credential_repo
            .expect_check_credentials_db()
            .once()
            .return_const(false);

        svc.token_repo.expect_save_token().never();

        let (username, password) = ("a".into(), "b".into());
        let credentials = Credentials { username, password };

        let actual = svc.login(credentials).await;
        assert_eq!(actual, None);
    }
}

// aux -----

async fn before_each() -> AuthService<MockCredentialRepoApi, MockTokenRepoApi> {
    AuthService::<MockCredentialRepoApi, MockTokenRepoApi>::new().await
}

impl AuthService<MockCredentialRepoApi, MockTokenRepoApi> {
    // TODO AuthService unit tests connect to DB and trigger empty TXs, since
    // actual queries are mocked out, but ideally they shouldn't need a DB.
    async fn new() -> Self {
        let cfg = app::extract_config("test");
        let db = db::connect(&cfg.db.url).await;
        let credential_repo = MockCredentialRepoApi::new();
        let token_repo = MockTokenRepoApi::new();
        Self {
            db,
            credential_repo,
            token_repo,
        }
    }
}