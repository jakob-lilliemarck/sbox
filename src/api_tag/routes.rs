extern crate futures;
extern crate serde;

use crate::db;
use crate::models::Tag;
use actix_web::{
    dev::HttpResponseBuilder, error, get, http::header, http::StatusCode, post, web, Error,
    HttpRequest, HttpResponse, Responder,
};
use derive_more::{Display, Error};
use futures::future::{ready, Ready};

type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/* ERRORHANDLING WIP */
#[derive(Debug, Display)]
pub enum ServerError {
    #[display(fmt = "internal error")]
    InternalError,
    #[display(fmt = "not found")]
    NotFound,
}

impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        println!("{:?}", self);
        match self {
            ServerError::InternalError => HttpResponse::InternalServerError().json("Server error."),
            ServerError::NotFound => HttpResponse::NotFound().json("Not found"),
        }
    }
}

impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> ServerError {
        match err {
            diesel::result::Error::NotFound => ServerError::NotFound,
            _ => ServerError::InternalError,
        }
    }
}
/* ENDOF ERRORHANDLING WIP */

#[get("/tags/{id}")]
pub async fn get_tags(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<HttpResponse, ServerError> {
    match db::read(
        &pool.get().expect("Could not connect to db from pool."),
        &id,
    ) {
        Ok(tag) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&tag).expect("json serialization error"))),
        Err(err) => {
            println!("ERR: {:?}", err);
            Err(err.into())
        }
    }
}

impl Responder for Tag {
    type Error = ServerError;
    type Future = Ready<Result<HttpResponse, ServerError>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        println!("BEFORE BODY SERIALIZATION");
        let body = serde_json::to_string(&self).unwrap();
        // Create response and set content typex§
        ready(Ok(HttpResponse::Ok()
            // TODO - common header shouled be set centrally.
            .content_type("application/json")
            .body("HEJ")))
    }
}

#[post("/tags")]
pub async fn create_tag(pool: web::Data<DbPool>, tag: web::Json<Tag>) -> impl Responder {
    db::create_if_none(
        &pool.get().expect("Could not connect to db from pool."),
        &tag,
    )
    .unwrap()
}
