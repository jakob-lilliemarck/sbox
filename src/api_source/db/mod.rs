use rocket_okapi::{
    gen::OpenApiGenerator, request::OpenApiFromRequest, request::RequestHeaderInput, OpenApiError,
};
use rocket_sync_db_pools::{database, diesel};

pub mod script;

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
