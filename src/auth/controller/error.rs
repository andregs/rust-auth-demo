use super::*;
    
impl From<Error> for Custom<Json<RestError>> {
    fn from(error: Error) -> Self {
        let id = Uuid::new_v4().to_string();
        let msg = error.to_string();
        let body = Json(RestError { id, msg });
        match error {
            Error::Duplicated(_) | Error::TooBig(_) => Custom(Status::BadRequest, body),
            Error::BadCredentials | Error::BadToken => Custom(Status::Unauthorized, body),
            Error::Other(source) => {
                // TODO proper logging
                eprintln!("Oops... {:?}", source);
                Custom(Status::InternalServerError, body)
            },
        }
    }
}
