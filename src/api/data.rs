use actix_web::{get, post, web, HttpResponse};
use sbox::errors::ServerError;
use sbox::utils::{get_conn, DbPool};

#[post("/data")]
pub async fn data_create<'a>(
    pool: web::Data<DbPool>,
    data: web::Json<SboxData>,
) -> Result<SboxData, ServerError<'a>> {}

#[get("/data/{id}")]
pub async fn data_get_id<'a>(
    pool: web::Data<DbPool>,
    id: i32
) -> Result<SboxData, ServerError<'a>> {}
