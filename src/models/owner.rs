use crate::errors::ServerError;
use crate::models::tag::Tag;
use crate::schema::{owner, owner_tag};

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

// 1:1 with the owner table
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "owner"]
pub struct Owner {
    pub id: i32,
    pub name: String,
}

// New owner form struct
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "owner"]
pub struct NewOwner {
    pub name: String,
}

// Follower -
#[derive(Debug, Deserialize, Serialize, Insertable, Associations, Identifiable, Queryable)]
#[table_name = "owner_tag"]
#[belongs_to(Owner)]
#[belongs_to(Tag)]
#[primary_key(owner_id, tag_id)]
pub struct Follower {
    pub owner_id: i32,
    pub tag_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewFollower {
    pub owner_id: i32,
}

impl NewFollower {
    pub fn to_follower<'a>(&self, tag_id: &'a i32) -> Follower {
        Follower {
            tag_id: tag_id.clone(),
            owner_id: self.owner_id,
        }
    }
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
