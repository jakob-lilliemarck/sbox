use crate::db;
use crate::models::source::{NewSource, Source};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Source")]
#[get("/source/<id>")]
pub async fn read_source(conn: db::Conn, id: i32) -> Json<Source> {
    let res = conn.run(move |c| db::source::read(c, &id)).await;
    Json(res.unwrap())
}

#[openapi(tag = "Source")]
#[post("/source", data = "<new_source>")]
pub async fn create_source(conn: db::Conn, new_source: Json<NewSource>) -> Json<Source> {
    let res = conn.run(move |c| db::source::create(c, &new_source)).await;
    Json(res)
}

#[openapi(tag = "Source")]
#[put("/source/<id>")]
pub async fn update_source(id: i32) {}

#[openapi(tag = "Source")]
#[delete("/source/<id>")]
pub async fn delete_source(id: i32) {}

use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
