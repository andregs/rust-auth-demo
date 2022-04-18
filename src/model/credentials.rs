use lazy_regex::regex_is_match;
use rocket::{async_trait, data, Request};
use rocket::data::{Data, FromData};
use rocket::outcome::Outcome::*;
use rocket::serde::{Deserialize, Serialize, json::Json};
use unicode_segmentation::UnicodeSegmentation;

use super::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl<'r> FromData<'r> for Credentials {
    type Error = HttpError;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use super::Error::*;

        let outcome = Json::<Credentials>::from_data(req, data).await
            .map(|json| json.into_inner())
            .map_failure(|(status, error)| (status, error.into()));

        // TODO cover these scenarios with tests
        if let Success(ref credentials) = outcome {
            let size = credentials.username.graphemes(true).count();
            let (min, max) = (1, 42);
            if size < min || size > max {
                return to_failure(BadUsernameSize(min, max));
            }

            let size = credentials.password.graphemes(true).count();
            let min = 8;
            if size < min {
                return to_failure(BadPasswordSize(min));
            }

            if ! regex_is_match!("^([A-Za-z]+)([0-9A-Za-z]*)$", &credentials.username) {
                return to_failure(BadUsername);
            }
        }

        outcome
    }
}

fn to_failure<'r>(error: Error) -> data::Outcome<'r, Credentials> {
    let error: HttpError = error.into();
    Failure((error.status(), error))
}
