extern crate sbox;

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sbox::schema::source;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, JsonSchema, Insertable)]
#[table_name = "source"]
pub struct NewSource {
    pub lang: String,
    pub src: String,
}

#[derive(Deserialize, Debug, JsonSchema, AsChangeset)]
#[table_name = "source"]
pub struct UpdateSource {
    lang: Option<String>,
    src: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    pub id: i32,
    pub lang: String,
    pub src: String,
}
