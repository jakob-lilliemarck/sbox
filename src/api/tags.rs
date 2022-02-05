use actix_web::{get, web};
use sbox::db::tag::read_by_owner;
use sbox::errors::ServerError;
use sbox::models::tag::TagList;
use sbox::utils::{get_conn, DbPool};

#[get("owners/{id}/tags")]
pub async fn tags_get_by_owner<'a>(
    pool: web::Data<DbPool>,
    owner_id: web::Path<i32>,
) -> Result<TagList, ServerError<'a>> {
    match read_by_owner(&get_conn(pool), &owner_id) {
        Ok(tags) => Ok(tags),
        Err(err) => Err(err.into()),
    }
}
