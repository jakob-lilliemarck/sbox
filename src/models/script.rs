use crate::db::tag::create_many;
use crate::errors::ServerError;
use crate::models::tag::{NewTag, Tag};
use crate::schema::script;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct TaggedNewScript {
    pub source: String,
    pub owner_id: i32,
    pub tags: Vec<String>, // this may need to be vec of i32.
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TaggedScript {
    pub id: i32,
    pub source: String,
    pub owner_id: i32,
    pub tags: Vec<String>,
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

/*
#[derive(Debug, Deserialize, Serialize)]
pub struct NewScriptAndTags(pub NewScript, pub Vec<NewTag>);

impl<'a> From<TaggedNewScript> for NewScriptAndTags {
    fn from(tagged_new_script: TaggedNewScript) -> NewScriptAndTags {
        let TaggedNewScript {
            source: source,
            owner_id,
            tags,
        } = tagged_new_script;
        // Instantiate new script:
        let new_script = NewScript {
            source: &source.to_owned(),
            owner_id,
        };
        // Instantiate new tags:
        let new_tags = tags
            .into_iter()
            .map(|value| NewTag {
                public: Some(false),
                value,
                owner_id,
            })
            .collect();
        // Return a tuple
        NewScriptAndTags(new_script, new_tags)
    }
}
*/
#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "script"]
pub struct Script {
    pub id: i32,
    pub source: String,
    pub owner_id: i32,
}

#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "script"]
pub struct NewScript<'a> {
    pub source: &'a str,
    pub owner_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct ScriptList {
    scripts: Vec<Script>,
}

impl From<Vec<Script>> for ScriptList {
    fn from(scripts: Vec<Script>) -> ScriptList {
        ScriptList { scripts }
    }
}

impl Responder for Script {
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

impl Responder for ScriptList {
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
