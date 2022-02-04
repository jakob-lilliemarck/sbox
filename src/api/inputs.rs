use actix_web::{get, post, put, web};

use sbox::db::inputs::{create, read};
use sbox::errors::ServerError;
use sbox::models::inputs::Input;
use sbox::utils::{get_conn, DbPool};

#[get("/inputs/{id}")]
pub async fn get_input<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<Input, ServerError<'a>> {
    match read(&get_conn(pool), &id) {
        Ok(input) => {
            println!("OK: {:?}", input);
            Ok(input)
        }
        Err(err) => {
            println!("ERR: {:?}", err);
            Err(err.into())
        }
    }
}

#[post("/inputs")]
pub async fn create_input<'a>(
    pool: web::Data<DbPool>,
    input: web::Json<Input>,
) -> Result<Input, ServerError<'a>> {
    match create(&get_conn(pool), &input) {
        Ok(input) => Ok(input),
        Err(err) => Err(err.into()),
    }
}
