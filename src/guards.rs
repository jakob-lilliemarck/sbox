use diesel::pg::PgConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use rocket::request::{self, FromRequest, Outcome, Request};
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::OpenApiError;
use std::env;

#[derive(Debug)]
pub struct DB;

impl DB {
    pub fn establish_connection(self) -> Result<PgConnection, &'static str> {
        dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
        Ok(PgConnection::establish(&database_url)
            .expect(&format!("Error connecting to {}", database_url)))
    }
}

#[derive(Debug)]
pub enum DbError {
    NoConnection,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for DB {
    type Error = DbError;
    async fn from_request(req: &'r Request<'_>) -> request::Outcome<Self, Self::Error> {
        println!("{:?}", req);
        Outcome::Success(DB)
    }
}

impl OpenApiFromRequest<'static> for DB {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        Ok(RequestHeaderInput::None)
    }
}
