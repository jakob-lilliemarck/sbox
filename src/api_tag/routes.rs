extern crate futures;
extern crate sbox;
extern crate serde;

use crate::db;
use crate::models::Tag;

use actix_web::{delete, dev::Body, get, post, web, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use sbox::errors::ServerError;
use sbox::utils::{get_conn, DbPool};

/* TODO: MAKE GENERIC*/
impl Responder for Tag {
    type Error = ServerError<'static>;
    type Future = Ready<Result<HttpResponse, ServerError<'static>>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        ready(Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(
                serde_json::to_string(&self).expect("Error serializing response"),
            )))
    }
}
/* ENDOF: MAKE GENERIC */

#[get("/tags/{id}")]
pub async fn get_tags<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<Tag, ServerError<'a>> {
    let test = &pool.get().expect("Could not connect to db from pool.");
    match db::read(&get_conn(pool), &id) {
        Ok(tag) => Ok(tag),
        Err(err) => Err(err.into()),
    }
}

#[post("/tags")]
pub async fn create_tag<'a>(
    pool: web::Data<DbPool>,
    tag: web::Json<Tag>,
) -> Result<Tag, ServerError<'a>> {
    match db::create_if_none(&get_conn(pool), &tag) {
        Ok(tag) => Ok(tag),
        Err(err) => Err(err.into()),
    }
}

#[delete("/tags/{id}")]
pub async fn delete_tag<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<HttpResponse, ServerError<'a>> {
    match db::delete(&get_conn(pool), &id) {
        Some(err) => Err(err.into()),
        None => Ok(HttpResponse::Ok().body(Body::Empty)),
    }
}
