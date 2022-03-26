use super::*;

#[cfg_attr(test, mockall::automock)]
pub trait CredentialRepoApi {
    fn save_credentials(&self, credentials: Credentials) -> bool {
        println!("Repo is saving {:?} in DB.", credentials);
        true
    }
}

pub struct PostgresCredentialRepo;

impl CredentialRepoApi for PostgresCredentialRepo {
    // yeap, empty. For now.
}
