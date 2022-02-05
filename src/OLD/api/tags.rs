extern crate futures;
extern crate sbox;
extern crate serde;

use sbox::db::tags::{create_if_none, delete, read};
use sbox::models::tags::Tag;

use actix_web::{delete, dev::Body, get, post, web, HttpResponse};
use sbox::errors::ServerError;
use sbox::utils::{get_conn, DbPool};

#[get("/tags/{id}")]
pub async fn get_tags<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<Tag, ServerError<'a>> {
    let test = &pool.get().expect("Could not connect to db from pool.");
    match read(&get_conn(pool), &id) {
        Ok(tag) => Ok(tag),
        Err(err) => Err(err.into()),
    }
}

#[post("/tags")]
pub async fn create_tag<'a>(
    pool: web::Data<DbPool>,
    tag: web::Json<Tag>,
) -> Result<Tag, ServerError<'a>> {
    match create_if_none(&get_conn(pool), &tag) {
        Ok(tag) => Ok(tag),
        Err(err) => Err(err.into()),
    }
}

#[delete("/tags/{id}")]
pub async fn delete_tag<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<HttpResponse, ServerError<'a>> {
    match delete(&get_conn(pool), &id) {
        Some(err) => Err(err.into()),
        None => Ok(HttpResponse::Ok().body(Body::Empty)),
    }
}
