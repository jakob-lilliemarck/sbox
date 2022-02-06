use actix_web::{delete, get, post, web, HttpResponse};
use diesel::result::Error;
use sbox::db::script::{create, create_tagged};
use sbox::db::tag::create_many;
use sbox::errors::ServerError;
use sbox::models::script::{NewScript, Script, TaggedNewScript, TaggedScript};
use sbox::models::tag::{NewTag, Tag};
use sbox::utils::{get_conn, DbPool};

pub struct ScriptWithTags(Script, Vec<Tag>);

fn convert_to_tuple(new_script: &TaggedNewScript) -> (NewScript, Vec<NewTag>) {
    let TaggedNewScript {
        source,
        owner_id,
        tags,
    } = new_script;
    let new_script = NewScript {
        source,
        owner_id: owner_id.clone(),
    };
    let new_tags = tags
        .into_iter()
        .map(|value| NewTag {
            public: Some(false),
            value: value.clone(),
            owner_id: owner_id.clone(),
        })
        .collect();
    (new_script, new_tags)
}

#[post("/scripts")]
pub async fn scripts_create<'a>(
    pool: web::Data<DbPool>,
    new_tagged_script: web::Json<TaggedNewScript>,
) -> Result<TaggedScript, ServerError<'a>> {
    let (new_script, new_tags) = convert_to_tuple(&new_tagged_script);
    match create_tagged(&get_conn(pool), &new_script, &new_tags) {
        Ok(script_and_tags) => Ok(TaggedScript::from(script_and_tags)),
        Err(err) => Err(err.into()),
    }
}

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
