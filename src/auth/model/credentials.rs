use lazy_regex::regex_is_match;
use rocket::{async_trait, data, Request};
use rocket::data::{Data, FromData};
use rocket::http::Status;
use rocket::outcome::Outcome::*;
use rocket::serde::{json, Deserialize, Serialize};
use rocket::serde::json::Json;
use rocket::response::status::Custom;
use unicode_segmentation::UnicodeSegmentation;

use super::*;

#[derive(Serialize, Deserialize, PartialEq, Eq, Debug, Clone)]
pub struct Credentials {
    pub username: String,
    pub password: String,
}

#[async_trait]
impl<'r> FromData<'r> for Credentials {
    type Error = Json<HttpError>;

    async fn from_data(req: &'r Request<'_>, data: Data<'r>) -> data::Outcome<'r, Self> {
        use super::Error::*;

        let outcome = Json::<Credentials>::from_data(req, data).await
            .map(|json| json.into_inner())
            .map_failure(|(status, error)| to_json_failure((status, error), req));

        // TODO cover these scenarios with tests
        if let Success(ref credentials) = outcome {
            let size = credentials.username.graphemes(true).count();
            let (min, max) = (1, 42);
            if size < min || size > max {
                return to_failure(BadUsernameSize(min, max), req);
            }

            let size = credentials.password.graphemes(true).count();
            let min = 8;
            if size < min {
                return to_failure(BadPasswordSize(min), req);
            }

            if ! regex_is_match!("^([A-Za-z]+)([0-9A-Za-z]*)$", &credentials.username) {
                return to_failure(BadUsername, req);
            }
        }

        outcome
    }
}

fn to_failure<'r>(error: Error, req: &'r Request<'_>) -> data::Outcome<'r, Credentials> {
    let error: Custom<Json<HttpError>> = error.into();
    let (status, json) = (error.0, error.1);
    req.local_cache(|| json.clone());
    return Failure((status, json));
}

fn to_json_failure<'r>((status, error): (Status, json::Error), req: &'r Request<'_>) -> (Status, Json<HttpError>) {
    let json = to_json(&error);
    req.local_cache(|| json.clone());
    (status, json)
}
