extern crate rocket;
use crate::models::source::Source;
use diesel::prelude::*;
use rocket::serde::json::Json;
use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::OpenApiError;
use rocket_sync_db_pools::{database, diesel};

#[database("db")]
pub struct Conn(diesel::PgConnection);

pub fn test(conn: &mut diesel::PgConnection) -> Result<Source, diesel::result::Error> {
    use crate::schema::source::dsl::*;
    source.find(id).first::<Source>(&*conn)
}

impl OpenApiFromRequest<'static> for Conn {
    fn from_request_input(
        gen: &mut OpenApiGenerator,
        name: String,
        required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        Ok(RequestHeaderInput::None)
    }
}
