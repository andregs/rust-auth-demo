use anyhow::anyhow;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::{async_trait, fairing::AdHoc};
use uuid::Uuid;

use super::*;

pub fn stage() -> AdHoc {
    AdHoc::on_request("Request Tracer", |req, _| {
        Box::pin(async move {
            let uuid = Uuid::new_v4().to_string();
            let mut tracer = Tracer::new();
            tracer.request_id = Some(uuid);
            req.local_cache(|| tracer);
        })
    })
}

#[derive(Default)]
pub struct Tracer {
    request_id: Option<String>,
}

impl Tracer {
    pub fn new() -> Self {
        Tracer { request_id: None }
    }

    pub fn request_id(&self) -> &String {
        self.request_id.as_ref().unwrap()
    }

    // TODO proper logger https://github.com/SergioBenitez/Rocket/issues/21

    pub fn info(&self, message: &str) {
        if let Some(id) = self.request_id.as_ref() {
            println!("{} INFO {}", id, message);
        }
    }

    pub fn error(&self, message: &str) {
        if let Some(id) = self.request_id.as_ref() {
            eprintln!("{} ERROR {}", id, message);
        }
    }
}

#[async_trait]
impl<'r> FromRequest<'r> for &'r Tracer {
    type Error = HttpError;

    async fn from_request(req: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let dummy = Tracer::new();
        let cached = req.local_cache(|| dummy);
        if cached.request_id.is_none() {
            let cause = anyhow!("Request ID not found. Did you attach the Tracer fairing?");
            return Error::Other(cause).into();
        }
        Outcome::Success(cached)
    }
}
