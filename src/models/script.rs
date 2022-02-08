use crate::errors::ServerError;
use crate::models::common::IdList;
use crate::schema::*;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "script"]
pub struct Script {
    pub id: i32,
    pub source: String,
    pub owner_id: i32,
}

#[derive(Debug, Deserialize, Insertable)]
#[table_name = "script"]
pub struct NewScript {
    pub source: String,
    pub owner_id: i32,
}

#[derive(Debug, Deserialize, AsChangeset, Insertable)]
#[table_name = "script"]
pub struct UpdateScript {
    pub source: String,
}

// Tagged script
#[derive(Debug, Deserialize, Serialize)]
pub struct TaggedScript {
    pub id: i32,
    pub source: String,
    pub owner_id: i32,
    pub tag_ids: IdList,
    pub output_tag_ids: IdList,
}

impl From<(Script, IdList, IdList)> for TaggedScript {
    fn from(script_tag_ids_tuple: (Script, IdList, IdList)) -> TaggedScript {
        let (s, tag_ids, output_tag_ids) = script_tag_ids_tuple;
        let Script {
            id,
            source,
            owner_id,
        } = s;
        TaggedScript {
            tag_ids,
            output_tag_ids,
            id,
            source,
            owner_id,
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct NewTaggedScript {
    pub source: String,
    pub tag_ids: Vec<i32>,
    pub output_tag_ids: Vec<i32>,
}

#[derive(Debug, Deserialize)]
pub struct UpdateTaggedScript {
    pub source: String,
    pub tags: Vec<i32>,
}

#[derive(Debug, Serialize)]
pub struct TaggedScriptList(pub Vec<TaggedScript>);

impl Responder for TaggedScript {
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

impl Responder for TaggedScriptList {
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
