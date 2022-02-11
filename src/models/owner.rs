use crate::errors::ServerError;
use crate::schema::owner;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "owner"]
pub struct Owner {
    pub id: i32,
    pub external_id: String,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "owner"]
pub struct NewOwner {
    pub external_id: String,
}

// TODO - make generic!
impl Responder for Owner {
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
