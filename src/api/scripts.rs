use actix_web::{delete, dev::Body, get, post, put, web, HttpResponse};
use sbox::db::script::{delete, read_tagged, read_tagged_by_owner};
use sbox::errors::ServerError;
use sbox::models::script::{NewTaggedScript, TaggedScript, TaggedScriptList, UpdateTaggedScript};
use sbox::utils::{get_conn, DbPool};

/*
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
*/

#[get("/scripts")]
pub async fn scripts_get_own<'a>(
    pool: web::Data<DbPool>,
) -> Result<TaggedScriptList, ServerError<'a>> {
    // TODO - owner_id should be derived from token in header during auth. Mocked to id = 1.
    match read_tagged_by_owner(&get_conn(pool), &1) {
        Ok(tagged_scripts) => Ok(tagged_scripts),
        Err(err) => Err(err.into()),
    }
}

#[get("/scripts/{id}")]
pub async fn scripts_get_id<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<TaggedScript, ServerError<'a>> {
    match read_tagged(&get_conn(pool), &id) {
        Ok(tagged_script) => Ok(tagged_script),
        Err(err) => Err(err.into()),
    }
}
/*
#[put("/scripts/{id}")]
pub async fn scripts_update<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
    tagged_script: web::Json<UpdateTaggedScript>,
) -> HttpResponse {
    update_tagged(&get_conn(pool), &tagged_script, &id);
    HttpResponse::Ok().body("OK")
}
*/
#[delete("/scripts/{id}")]
pub async fn scripts_delete<'a>(
    pool: web::Data<DbPool>,
    id: web::Path<i32>,
) -> Result<HttpResponse, ServerError<'a>> {
    match delete(&get_conn(pool), &id) {
        Ok(_) => Ok(HttpResponse::Ok().body(Body::Empty)),
        Err(err) => Err(err.into()),
    }
}
