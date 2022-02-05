use actix_web::{get, post, delete, web, HttpResponse};
use sbox::errors::ServerError;
use sbox::utils::{get_conn, DbPool};

#[post("/scripts")]
pub async fn scripts_create<'a>(
    pool: web::Data<DbPool>,
    data: web::Json<SboxScript>,
) -> Result<SboxScript, ServerError<'a>> {}

/*
#[get("/scripts")]
pub async fn scripts_get<'a>(
    pool: web::Data<DbPool>,
) -> Result<Vec<SboxScript>, ServerError<'a>> {}

#[get("/scripts/{id}")]
pub async fn scripts_get_id<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>
) -> Result<SboxScript, ServerError<'a>> {}

#[delete("/scripts/{id}")]
pub async fn scripts_delete<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>
) -> Result<HttpResponse, ServerError<'a>>
*/
