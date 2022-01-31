use crate::db::test::{test, Conn};
use crate::models::source::Source;
use crate::schema;
use diesel::prelude::*;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::openapi;
use rocket_sync_db_pools::diesel;

#[openapi(tag = "Source")]
#[get("/source/<id>")]
pub async fn read_source(conn: Conn, id: i32) /*-> Result<Json<Source>, Status>*/
{
    let r = conn.run(|c| test(c)).await;
    println!("{}", r);
    //let s = find(&conn, id);
    //println!("{:?}", s)

    /*let s = source.find(7).first::<Source>(&conn);
    match s {
        Ok(s) => Ok(Json(s)),
        Err(err) => match err {
            _ => Err(Status::NotFound),
        },
    }*/
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
