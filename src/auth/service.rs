use super::*;

pub trait AuthServiceApi {
    fn register(&self, credentials: Credentials) -> bool;
}

pub struct AuthService<CR = PostgresCredentialRepo>
    where CR: CredentialRepoApi,
{
    pub credential_repo: CR,
}

impl<CR> AuthService<CR>
    where CR: CredentialRepoApi
{
    pub fn new(credential_repo: CR) -> Self {
        Self { credential_repo }
    }
}

impl <CR> AuthServiceApi for AuthService<CR>
    where CR: CredentialRepoApi + Sync + Send {

    fn register(self: &Self, credentials: Credentials) -> bool {
        self.credential_repo.save_credentials(credentials)
    }
}
