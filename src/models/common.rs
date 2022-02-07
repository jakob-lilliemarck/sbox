use crate::errors::ServerError;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct IdList(pub Vec<i32>);

impl Responder for IdList {
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
