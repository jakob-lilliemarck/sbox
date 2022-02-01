use crate::schema::source;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/*
#[derive(Deserialize, Debug, JsonSchema, Insertable)]
#[table_name = "source"]
pub struct NewSource<'a> {
    pub lang: &'a str,
    pub src: &'a str,
}
*/
#[derive(Deserialize, Debug, JsonSchema, Insertable)]
#[table_name = "source"]
pub struct NewSource {
    pub lang: String,
    pub src: String,
}

#[derive(Queryable, Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub id: i32,
    pub lang: String,
    pub src: String,
}
