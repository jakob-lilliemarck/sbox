extern crate sbox;

use sbox::schema::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "input"]
pub struct Input {
    pub id: i32,
    pub data: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable, Associations)]
#[belongs_to(Input)]
#[table_name = "input_tag"]
pub struct InputTag {
    pub input_id: i32,
    pub tag_id: String,
}

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "tag"]
pub struct Tag {
    pub id: String,
}
