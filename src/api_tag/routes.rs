use crate::db;
use crate::models::Tag;
use actix_web::{get, post, web, HttpResponse, Responder};

type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[post("/tags")]
pub async fn create_tag(pool: web::Data<DbPool>) -> impl Responder {
    let conn = pool.get().expect("Could not connect to db from pool.");
    let new_tag = Tag {
        id: "TEST".to_string(),
    };
    println!("TESTTEST");
    let res = db::create(&conn, &new_tag);
    println!("CREATED: {:?}", res);
    HttpResponse::Ok().body("OKOK!")
}

#[get("/tags/{id}")]
pub async fn get_tags(pool: web::Data<DbPool>, id: web::Path<(String)>) -> impl Responder {
    println!("RECIEVED ID: {:?}", id);
    let conn = pool.get().expect("Could not connect to db from pool.");
    let res = db::read(&conn, &id);
    println!("FOUND: {:?}", res);
    HttpResponse::Ok().body("OK!")
}
