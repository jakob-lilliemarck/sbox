use crate::errors::ServerError;
use crate::schema::{data, data_tag};

use actix_web::{HttpRequest, HttpResponse, Responder};
use futures::future::{ready, Ready};
use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Queryable, Identifiable)]
#[table_name = "data"]
pub struct Data {
    id: i32,
    value: String,
    input_id: Option<i32>,
    script_id: Option<i32>,
}

#[derive(Debug, Serialize, Deserialize, Insertable, Identifiable, Associations)]
#[belongs_to(Data)]
#[primary_key(data_id, tag_id)]
#[table_name = "data_tag"]
pub struct DataTag {
    data_id: i32,
    tag_id: i32,
}
