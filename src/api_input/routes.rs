extern crate sbox;

use crate::db;
use crate::models::{Input, Tag};

use actix_web::{get, post, put, web, HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use sbox::errors::ServerError;
use sbox::utils::{get_conn, DbPool};

/* TODO: MAKE GENERIC*/
impl Responder for Input {
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

#[get("/inputs/{id}")]
pub async fn get_input<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<Input, ServerError<'a>> {
    match db::read(&get_conn(pool), &id) {
        Ok(input) => {
            println!("OK: {:?}", input);
            Ok(input)
        }
        Err(err) => {
            println!("ERR: {:?}", err);
            Err(err.into())
        }
    }
}

#[put("/inputs/{id}/tag")]
pub async fn tag_input<'a>(pool: web::Data<DbPool>, tag: web::Json<Tag>)-> Result<Input, ServerError<'a>> {
    // add a tag to an input - this should ideally be done on POST, not put
}

#[post("/inputs")]
pub async fn create_input<'a>(
    pool: web::Data<DbPool>,
    input: web::Json<Input>,
) -> Result<Input, ServerError<'a>> {
    match db::create(&get_conn(pool), &input) {
        Ok(input) => Ok(input),
        Err(err) => Err(err.into()),
    }
}
