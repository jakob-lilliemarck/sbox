extern crate sbox;

use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use sbox::schema::script;
use serde::{Deserialize, Serialize};

#[derive(Deserialize, Debug, JsonSchema, Insertable)]
#[table_name = "script"]
pub struct NewScript {
    pub lang: String,
    pub source: String,
}

#[derive(Deserialize, Debug, JsonSchema, AsChangeset)]
#[table_name = "script"]
pub struct UpdateScript {
    lang: Option<String>,
    source: Option<String>,
}

#[derive(Queryable, Deserialize, Serialize, JsonSchema, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Script {
    pub id: i32,
    pub lang: String,
    pub source: String,
}
