use crate::db;
use crate::models::source::{NewSource, Source, UpdateSource};
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use sbox::tasks::CeleryAppInstance;

#[openapi(tag = "Source")]
#[get("/source/<id>")]
pub async fn read_source(
    conn: db::Conn,
    id: i32,
    celery: CeleryAppInstance,
) -> Result<Json<Source>, Status> {
    celery.app.send_task(sbox::tasks::add::new(1, 2)).await;

    let res = conn.run(move |c| db::source::read(c, &id)).await;
    match res {
        Ok(source) => Ok(Json(source)),
        Err(_err) => Err(Status::NotFound),
    }
}

#[openapi(tag = "Source")]
#[post("/source", data = "<new_source>")]
pub async fn create_source(
    conn: db::Conn,
    new_source: Json<NewSource>,
) -> Result<Json<Source>, Status> {
    let res = conn.run(move |c| db::source::create(c, &new_source)).await;
    match res {
        Ok(source) => Ok(Json(source)),
        Err(_err) => Err(Status::InternalServerError),
    }
}

#[openapi(tag = "Source")]
#[put("/source/<id>", data = "<update_source>")]
pub async fn update_source(
    conn: db::Conn,
    id: i32,
    update_source: Json<UpdateSource>,
) -> Result<Json<Source>, Status> {
    let res = conn
        .run(move |c| db::source::update(c, &id, &update_source))
        .await;
    match res {
        Ok(source) => Ok(Json(source)),
        Err(_err) => Err(Status::InternalServerError),
    }
}

#[openapi(tag = "Source")]
#[delete("/source/<id>")]
pub async fn delete_source(conn: db::Conn, id: i32) -> Result<Status, Status> {
    let res = conn.run(move |c| db::source::delete(c, &id)).await;
    match res {
        Ok(_num) => Ok(Status::Ok),
        Err(_err) => Err(Status::InternalServerError),
    }
}

#[openapi(tag = "test")]
#[get("/test")]
pub async fn test() {
    println!("TEST")
}
