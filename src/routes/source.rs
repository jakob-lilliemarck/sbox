use crate::db;
use crate::models::source::NewSource;
use rocket::serde::json::Json;
use rocket_okapi::openapi;

#[openapi(tag = "Source")]
#[get("/source/<id>")]
pub async fn read_source(conn: db::Conn, id: i32) {
    let r = conn.run(move |c| db::source::read(c, &id)).await;
    match r {
        Ok(source) => println!("OK, FOUND!"),
        Err(err) => println!("ERR ERR ERR"),
    }
}

#[openapi(tag = "Source")]
#[post("/source", data = "<new_source>")]
pub async fn create_source(conn: db::Conn, new_source: Json<NewSource<'_>>) {
    println!("IN ROUTE: {:?}", new_source.lang);
    let r = conn.run(move |c| db::source::create(c, &new_source)).await;
}

#[openapi(tag = "Source")]
#[put("/source/<id>")]
pub async fn update_source(id: i32) {}

#[openapi(tag = "Source")]
#[delete("/source/<id>")]
pub async fn delete_source(id: i32) {}
