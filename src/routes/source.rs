use crate::models::source::Source;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Source")]
#[get("/source/<id>")]
pub fn read_source(id: i32) {}

#[openapi(tag = "Source")]
#[post("/source")]
pub fn create_source() {}

#[openapi(tag = "Source")]
#[put("/source/<id>")]
pub fn update_source(id: i32) {}

#[openapi(tag = "Source")]
#[delete("/source/<id>")]
pub fn delete_source(id: i32) {}
