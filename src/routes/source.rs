use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

use crate::guards::DB;
use crate::models::source::Source;

#[openapi(tag = "Source")]
#[get("/source/<id>")]
pub fn read_source(id: i32, _db: DB) -> Result<Json<Source>, rocket::http::Status> {
    use crate::schema::source::dsl::*;
    let connection = _db.establish_connection().unwrap();
    let s = source.find(id).first::<Source>(&connection);
    match s {
        Ok(s) => Ok(Json(s)),
        Err(err) => match err {
            _ => Err(Status::NotFound),
        },
    }
}

#[openapi(tag = "Source")]
#[post("/source")]
pub fn create_source() {}

#[openapi(tag = "Source")]
#[put("/source/<id>")]
pub fn update_source(id: i32) {}

#[openapi(tag = "Source")]
#[delete("/source/<id>")]
pub fn delete_source(id: i32) {}
