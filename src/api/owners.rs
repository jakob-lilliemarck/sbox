use actix_web::{post, web};
use sbox::db::owner::create;
use sbox::errors::ServerError;
use sbox::models::owner::{NewOwner, Owner};
use sbox::utils::{get_conn, DbPool};

#[post("/owners")]
pub async fn owner_create<'a>(
    pool: web::Data<DbPool>,
    new_owner: web::Json<NewOwner>,
) -> Result<Owner, ServerError<'a>> {
    match create(&get_conn(pool), &new_owner) {
        Ok(owner) => Ok(owner),
        Err(err) => Err(err.into()),
    }
}
