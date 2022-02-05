use crate::errors::ServerError;
use crate::models::tags::Tag;
use crate::schema::*;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Insertable)]
#[table_name = "input"]
pub struct NewInput {
    pub data: String,
}

impl NewInput {
    /*pub fn from_new_tagged_input(new_tagged_input: &NewTaggedInput) -> NewInput {
        NewInput {
            data: new_tagged_input.clone().data,
        }
    }*/
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTaggedInput {
    pub data: String,
    pub tags: Vec<String>,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Identifiable)]
#[table_name = "input"]
pub struct Input {
    pub id: i32,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Associations, Identifiable)] // Identifiable
#[belongs_to(Input)]
#[primary_key(input_id, tag_id)]
#[table_name = "input_tag"]
pub struct InputTag {
    pub input_id: i32,
    pub tag_id: String,
}

/*
impl Into<(NewInput, Vec<Tag>)> for NewTaggedInput {
    fn into(self) -> (NewInput, Vec<Tag>) {
        (
            NewInput { data: self.data },
            self.tags.into_iter().map(|tag| Tag { id: tag }).collect(),
        )
    }
}
*/

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
