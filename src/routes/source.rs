use crate::db;
use crate::models::source::{NewSource, Source, UpdateSource};
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
#[put("/source/<source_id>", data = "<update_source>")]
pub async fn update_source(conn: db::Conn, source_id: i32, update_source: Json<UpdateSource>) {
    let res = conn
        .run(move |c| db::source::update(c, &source_id, &update_source))
        .await;
}

#[openapi(tag = "Source")]
#[delete("/source/<source_id>")]
pub async fn delete_source(conn: db::Conn, source_id: i32) {
    let res = conn.run(move |c| db::source::delete(c, &source_id)).await;
}

use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
