use crate::errors::ServerError;
use crate::models::common::IdList;
use crate::models::script::Script;
use crate::models::tag::Tag;
use crate::schema::script_tag;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Insertable, Identifiable, Associations, Queryable)]
#[belongs_to(Script)]
#[belongs_to(Tag)]
#[primary_key(script_id, tag_id)]
#[table_name = "script_tag"]
pub struct ScriptTag {
    pub script_id: i32,
    pub tag_id: i32,
}

impl ScriptTag {
    pub fn to_tuple_id(&self) -> (&i32, &i32) {
        (&self.script_id, &self.tag_id)
    }
}

// TODO - make generic!
impl Responder for ScriptTag {
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
