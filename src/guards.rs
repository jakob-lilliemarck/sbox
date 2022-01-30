use rocket::request::{self, FromRequest, Outcome, Request};
use rocket::serde::json::Json;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::OpenApiError;

pub struct DB<'r>(&'r str);

#[derive(Debug)]
pub enum DbError {
    NoConnection,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DB<'r> {
    type Error = DbError;
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        println!("TEST IN GUARD");
        Outcome::Success(DB("hej"))
    }
}

impl OpenApiFromRequest<'static> for DB<'static> {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        Ok(RequestHeaderInput::None)
    }
}
