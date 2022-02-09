use crate::errors::ServerError;
use crate::models::common::IdList;
use crate::models::tag::Tag;
use crate::schema::*;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "script"]
pub struct Script {
    pub id: i32,
    pub source: String,
    pub owner_id: Option<i32>,
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

#[derive(Debug, Deserialize, AsChangeset, Insertable)]
#[table_name = "script"]
#[changeset_options(treat_none_as_null = "true")]
pub struct UpdateScriptOwner {
    pub owner_id: Option<i32>,
}

// Tagged script - as represented in the system
#[derive(Debug, Deserialize, Serialize)]
pub struct TaggedScript {
    pub id: i32,
    pub source: String,
    pub owner_id: Option<i32>,
    pub tags: Vec<Tag>,
    pub output_tags: Vec<Tag>,
}

// Tagged script - as represented to the user
#[derive(Debug, Deserialize, Serialize)]
pub struct StrippedTaggedScript {
    pub id: i32,
    pub source: String,
    pub owner_id: Option<i32>,
    pub tag_ids: IdList,
    pub output_tag_ids: IdList,
}

impl From<(Script, IdList, IdList)> for StrippedTaggedScript {
    fn from(script_tag_tuple: (Script, IdList, IdList)) -> StrippedTaggedScript {
        let (s, tag_ids, output_tag_ids) = script_tag_tuple;
        let Script {
            id,
            source,
            owner_id,
        } = s;
        StrippedTaggedScript {
            tag_ids,
            output_tag_ids,
            id,
            source,
            owner_id,
        }
    }
}

impl From<TaggedScript> for StrippedTaggedScript {
    fn from(tagged_script: TaggedScript) -> StrippedTaggedScript {
        let TaggedScript {
            tags,
            output_tags,
            id,
            source,
            owner_id,
        } = tagged_script;

        StrippedTaggedScript {
            tag_ids: tags.into(),
            output_tag_ids: output_tags.into(),
            id,
            source,
            owner_id,
        }
    }
}

impl From<(Script, Vec<(Tag, bool)>)> for TaggedScript {
    fn from(script_tag_tuple: (Script, Vec<(Tag, bool)>)) -> TaggedScript {
        let (s, t) = script_tag_tuple;
        let (tags, output_tags) = t.iter().fold((vec![], vec![]), |mut accumulator, tup| {
            if tup.1 {
                accumulator.1.push(tup.0.clone());
                accumulator
            } else {
                accumulator.0.push(tup.0.clone());
                accumulator
            }
        });
        let Script {
            id,
            source,
            owner_id,
        } = s;
        TaggedScript {
            tags,
            output_tags,
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

impl Responder for StrippedTaggedScript {
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
