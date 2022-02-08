use crate::errors::ServerError;
use crate::models::script::Script;
use crate::models::tag::Tag;
use crate::schema::script_tag;

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Insertable, Deserialize, Identifiable, Associations, Queryable)]
#[belongs_to(Script)]
#[belongs_to(Tag)]
#[primary_key(script_id, tag_id)]
#[table_name = "script_tag"]
pub struct ScriptTag {
    pub script_id: i32,
    pub tag_id: i32,
    pub is_output: bool,
}

#[derive(Debug, Serialize, Insertable, Deserialize, AsChangeset)]
#[table_name = "script_tag"]
pub struct UpdateScriptTag {
    pub is_output: bool,
}

impl ScriptTag {
    pub fn to_tuple_id(&self) -> (&i32, &i32) {
        (&self.script_id, &self.tag_id)
    }
    pub fn to_update_script_tag(&self) -> UpdateScriptTag {
        UpdateScriptTag {
            is_output: self.is_output.clone(),
        }
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
