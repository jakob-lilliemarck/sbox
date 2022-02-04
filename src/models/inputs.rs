use crate::errors::ServerError;
use crate::schema::*;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "input"]
pub struct Input {
    pub id: i32,
    pub data: String,
}

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

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[belongs_to(Input)]
#[table_name = "input_tag"]
pub struct InputTag {
    pub input_id: i32,
    pub tag_id: String,
}
