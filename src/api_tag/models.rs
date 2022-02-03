extern crate sbox;

use sbox::schema::tag;
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Insertable)]
#[table_name = "tag"]
pub struct Tag {
    pub id: String,
}
