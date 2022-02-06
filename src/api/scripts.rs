use actix_web::{get, post, web};
use sbox::db::script::{create_tagged, read_by_owner};
use sbox::errors::ServerError;
use sbox::models::script::{NewScriptAndTags, NewTaggedScript, TaggedScript, TaggedScriptList};
use sbox::utils::{get_conn, DbPool};

#[post("/scripts")]
pub async fn scripts_create<'a>(
    pool: web::Data<DbPool>,
    new_tagged_script: web::Json<NewTaggedScript>, // could implement conversion already here.
) -> Result<TaggedScript, ServerError<'a>> {
    let NewScriptAndTags(new_script, new_tags) = NewScriptAndTags::from(&*new_tagged_script);
    match create_tagged(&get_conn(pool), &new_script, &new_tags) {
        Ok(script_and_tags) => Ok(TaggedScript::from(script_and_tags)),
        Err(err) => Err(err.into()),
    }
}

#[get("/scripts")]
pub async fn scripts_get_own<'a>(
    pool: web::Data<DbPool>,
) -> Result<TaggedScriptList, ServerError<'a>> {
    match read_by_owner(&get_conn(pool), &1) {
        Ok(tagged_scripts) => Ok(TaggedScriptList(tagged_scripts)),
        Err(err) => Err(err.into()),
    }
}
/*
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
