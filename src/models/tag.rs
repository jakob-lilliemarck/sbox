use crate::errors::ServerError;
use crate::schema::tag;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable, AsChangeset)]
#[table_name = "tag"]
pub struct Tag {
    pub id: i32,
    pub value: String,
    pub is_public: bool,
    pub owner_id: Option<i32>,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "tag"]
pub struct NewTag {
    pub value: String,
    pub is_public: Option<bool>,
}

#[derive(Debug, Deserialize, AsChangeset, Insertable)]
#[table_name = "tag"]
pub struct UpdateTag {
    pub is_public: Option<bool>,
}

#[derive(Debug, Insertable, AsChangeset)]
#[table_name = "tag"]
#[changeset_options(treat_none_as_null = "true")]
pub struct UpdateTagOwner {
    pub owner_id: Option<i32>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagList(pub Vec<Tag>);

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

impl Responder for TagList {
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
