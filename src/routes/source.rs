use crate::db;
use crate::models::source::{NewSource, Source};
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Source")]
#[get("/source/<id>")]
pub async fn read_source(conn: db::Conn, id: i32) -> Json<Source> {
    let r = conn.run(move |c| db::source::read(c, &id)).await;
    Json(r.unwrap())
}
/*
#[openapi(tag = "Source")]
#[post("/source", format = "json", data = "<new_source>")]
pub async fn create_source(conn: db::Conn, new_source: Json<NewSource<'_>>) -> String {
    println!("IN ROUTE: {:?}", new_source);
    //let r = conn.run(move |c| db::source::create(c, &new_source)).await;
    String::from("HEJ")
}
*/

#[openapi(tag = "Source")]
#[post("/source", data = "<new_test>")]
pub async fn create_source(conn: db::Conn, new_test: Json<NewSource>) -> String {
    use crate::schema::source;
    let res = conn.run(move |c| db::source::create(c, &new_test)).await;

    /*
    let res = conn
        .run(|c| {
            diesel::insert_into(source::table)
                .values(new_test.into_inner())
                .execute(c)
        })
        .await;
    println!("DEBUG RES: {:?}", res.unwrap());*/

    String::from("success")
}

#[openapi(tag = "Source")]
#[put("/source/<id>")]
pub async fn update_source(id: i32) {}

#[openapi(tag = "Source")]
#[delete("/source/<id>")]
pub async fn delete_source(id: i32) {}

use diesel::prelude::*;
use rocket_sync_db_pools::diesel;
