extern crate futures;
extern crate serde;

use crate::db;
use crate::models::Tag;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use derive_more::{Display, Error};
use futures::future::{ready, Ready};
use serde::Serialize;

type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

/* ERRORHANDLING WIP */
#[derive(Debug, Display)]
pub enum ServerError {
    #[display(fmt = "Not found")]
    NotFound,
    #[display(fmt = "Bad request")]
    BadRequest(String),
    #[display(fmt = "Unknown error")]
    UnknownError,
}

impl ServerError {
    pub fn name(&self) -> &str {
        match self {
            Self::NotFound => "Not found",
            Self::BadRequest(_data) => "Bad request",
            Self::UnknownError => "Unknown error",
        }
    }

    // Methods to format custom error responses.
    // TODO - Consider returning an Option<> type here instead, to be able to skip message in
    // json body on None
    pub fn message(&self) -> &str {
        match self {
            Self::BadRequest(message) => message,
            Self::NotFound => "Not found",
            _ => "",
        }
    }
}

// Cast diesel::result::Error into a ServerError depending on type of diesel::result::Error.
// Doing this for the errors that may occur effectivly "unifies" them to be of the same enum
// which is needed since the ResponseError trait mush be implemented on an enum defined in the
// same scope. It's also much nicer to implement it for one enum, over many varied kinds.
impl From<diesel::result::Error> for ServerError {
    fn from(err: diesel::result::Error) -> ServerError {
        match err {
            diesel::result::Error::NotFound => ServerError::NotFound,
            _ => ServerError::UnknownError,
        }
    }
}

// json response object for errors
#[derive(Serialize)]
struct ErrorResponse<'r> {
    error: &'r str,
    message: &'r str,
}

// Errorhandler for ServerError, provides basic control-flow and responses depending on type.
impl actix_web::error::ResponseError for ServerError {
    fn error_response(&self) -> HttpResponse {
        let status_code = self.status_code();
        let error_response = ErrorResponse {
            message: self.message(),
            error: self.name(),
        };
        HttpResponse::build(status_code).json(error_response)
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
        // Make a response builder & some way to set default headers
        Ok(tag) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&tag).expect("json serialization error"))),
        // casts the diesel::error::Error returned by read() into() an ServerError (see
        // func signature).
        Err(err) => Err(err.into()),
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
