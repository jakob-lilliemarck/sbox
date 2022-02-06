use crate::errors::ServerError;
use crate::models::tag::{NewTag, Tag};
use crate::schema::*;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

// 1:1 with the sql schema
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "script"]
pub struct Script {
    pub id: i32,
    pub source: String,
    pub owner_id: i32,
}

// Whats inserted in thr script-table on creation
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "script"]
pub struct NewScript {
    pub source: String,
    pub owner_id: i32,
}

// The resource send in response to the client
#[derive(Debug, Deserialize, Serialize)]
pub struct TaggedScript {
    pub id: i32,
    pub source: String,
    pub owner_id: i32,
    pub tags: Vec<String>,
}

// The resource the client posts/puts to the api
#[derive(Debug, Deserialize, Serialize)]
pub struct NewTaggedScript {
    pub source: String,
    pub owner_id: i32,
    pub tags: Vec<String>,
}

// Intermediary conversion type to split TaggedScript to db table-row elements
pub struct NewScriptAndTags(pub NewScript, pub Vec<NewTag>);

impl<'a> From<&NewTaggedScript> for NewScriptAndTags {
    fn from(new_tagged_script: &NewTaggedScript) -> NewScriptAndTags {
        let NewTaggedScript {
            source,
            owner_id,
            tags,
        } = new_tagged_script;

        let new_script = NewScript {
            source: source.clone(),
            owner_id: owner_id.clone(),
        };
        let new_tags = tags
            .into_iter()
            .map(|value| NewTag {
                public: Some(false),
                value: value.clone(),
                owner_id: owner_id.clone(),
            })
            .collect();
        NewScriptAndTags(new_script, new_tags)
    }
}

impl From<(Script, Vec<Tag>)> for TaggedScript {
    fn from(script_and_tags: (Script, Vec<Tag>)) -> TaggedScript {
        let (script, tags) = script_and_tags;
        TaggedScript {
            id: script.id,
            source: script.source,
            owner_id: script.owner_id,
            tags: tags.into_iter().map(|tag| tag.value).collect(),
        }
    }
}

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

// A struct to implement Responder on when returning an array of TaggedScripts
#[derive(Debug, Deserialize, Serialize)]
pub struct TaggedScriptList(pub Vec<TaggedScript>);

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

#[derive(Debug, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable)]
#[belongs_to(Script)]
#[belongs_to(Tag)]
#[primary_key(script_id, tag_id)]
#[table_name = "script_tag"]
pub struct ScriptTag {
    pub script_id: i32,
    pub tag_id: i32,
}
