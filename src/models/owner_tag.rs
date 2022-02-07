use crate::errors::ServerError;
use crate::models::owner::Owner;
use crate::models::tag::Tag;
use crate::schema::owner_tag;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Insertable, Associations, Identifiable, Queryable)]
#[table_name = "owner_tag"]
#[belongs_to(Owner)]
#[belongs_to(Tag)]
#[primary_key(owner_id, tag_id)]
pub struct Follower {
    pub owner_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Deserialize)]
pub struct NewFollower {
    pub owner_id: i32,
}

// TODO - make generic!
impl Responder for Follower {
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
