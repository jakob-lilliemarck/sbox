use rocket_okapi::gen::OpenApiGenerator;
use rocket_okapi::request::{OpenApiFromRequest, RequestHeaderInput};
use rocket_okapi::OpenApiError;
use rocket_sync_db_pools::{database, diesel};

pub mod source;

#[database("db")]
pub struct Conn(diesel::PgConnection);

impl OpenApiFromRequest<'static> for Conn {
    fn from_request_input(
        _gen: &mut OpenApiGenerator,
        _name: String,
        _required: bool,
    ) -> Result<RequestHeaderInput, OpenApiError> {
        Ok(RequestHeaderInput::None)
    }
}
