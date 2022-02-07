use crate::errors::ServerError;
use crate::schema::tag;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "tag"]
pub struct Tag {
    pub id: i32,
    pub value: String,
    pub public: bool,
    pub owner_id: Option<i32>,
}

// NEW
#[derive(Debug, Deserialize, Serialize, Insertable)]
#[table_name = "tag"]
pub struct NewTag {
    pub value: String,
    pub public: Option<bool>, // will it work?
    pub owner_id: i32,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct NewTagList {
    new_tags: Vec<NewTag>,
}

impl From<Vec<NewTag>> for NewTagList {
    fn from(new_tags: Vec<NewTag>) -> NewTagList {
        NewTagList { new_tags }
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct TagList {
    tags: Vec<Tag>,
}

impl From<Vec<Tag>> for TagList {
    fn from(tags: Vec<Tag>) -> TagList {
        TagList { tags }
    }
}

// TODO - make generic
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
