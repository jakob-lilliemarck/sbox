use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Deserialize, Serialize, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub id: i32,
    pub lang: &'static str,
    pub src: &'static str,
}
