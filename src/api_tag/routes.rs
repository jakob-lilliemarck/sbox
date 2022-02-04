extern crate futures;
extern crate serde;

use crate::db;
use crate::errors::ServerError;
use crate::models::Tag;
use actix_web::{get, post, web, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
type DbPool = r2d2::Pool<diesel::r2d2::ConnectionManager<diesel::PgConnection>>;

#[get("/tags/{id}")]
pub async fn get_tags<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<String>,
) -> Result<HttpResponse, ServerError<'a>> {
    match db::read(
        &pool.get().expect("Could not connect to db from pool."),
        &id,
    ) {
        // Make a response builder & some way to set default headers
        Ok(tag) => Ok(HttpResponse::Ok()
            .content_type("application/json")
            .body(serde_json::to_string(&tag).expect("json serialization error"))),
        //.body(tag)), TODO - this will work if creating custom response struct similar to errorhandling <=== !
        // casts the diesel::error::Error returned by read() into() an ServerError (see
        // func signature).
        Err(err) => Err(err.into()), // Err(ServerError::BadRequest(None)) example of custom cause
    }
}

// TODO - remove dependency on responder impl? Or see how this may go together with error handling
/*impl Responder for Tag {
    type Error = ServerError;
    type Future = Ready<Result<HttpResponse, ServerError>>;

    fn respond_to(self, _req: &HttpRequest) -> Self::Future {
        println!("BEFORE BODY SERIALIZATION");
        let body = serde_json::to_string(&self).unwrap();
        // Create response and set content typex§
        ready(Ok(HttpResponse::Ok()
            // TODO - common header shouled be set centrally.
            .content_type("application/json")
            .body(body)))
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
*/
