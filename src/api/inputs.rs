use actix_web::{get, post, put, web, HttpResponse};

use sbox::db::inputs::{create, create_tagged_input, read};
use sbox::errors::ServerError;
use sbox::models::inputs::{Input, InputTag, NewTaggedInput};
use sbox::models::tags::Tag;
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

#[put("/inputs/{id}/tags")]
pub async fn input_tag(
    pool: web::Data<DbPool>,
    tag: web::Json<Tag>,
    input_id: web::Path<i32>,
) -> HttpResponse {
    let input_tag = InputTag {
        input_id: *input_id,
        tag_id: tag.id.to_string(),
    };

    //let test = sbox::db::inputs::tag(&get_conn(pool), &input_tag);
    //println!("TEST: {:?}", test);

    HttpResponse::Ok().body("OK")
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

#[post("/in")]
pub async fn create_input_and_tags(
    pool: web::Data<DbPool>,
    tagged_input: web::Json<NewTaggedInput>,
) -> HttpResponse {
    println!("TEST: {:?}", tagged_input);
    create_tagged_input(&get_conn(pool), &tagged_input);
    // create input
    // create each tag
    // associate
    // return
    //println!("FROM: {:?}", InputTagTup::from(tagged_input));
    HttpResponse::Ok().body("OK!")
}
