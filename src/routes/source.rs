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
pub fn create_source(conn: db::Conn, new_source: Json<NewSource<'_>>) {
    println!("IN ROUTE: {:?}", new_source.lang);
    //let r = conn.run(|c| db::source::create(c));
}

#[openapi(tag = "Source")]
#[put("/source/<id>")]
pub fn update_source(id: i32) {}

#[openapi(tag = "Source")]
#[delete("/source/<id>")]
pub fn delete_source(id: i32) {}
