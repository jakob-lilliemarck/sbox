use actix_web::{delete, dev::Body, get, post, put, web, HttpResponse};
use serde::{Deserialize, Serialize};

use sbox::db::script;
use sbox::db::script_tag;
use sbox::db::tag;
use sbox::errors::ServerError;
use sbox::models::owner::Owner;
use sbox::models::script::{
    NewTaggedScript, Script, TaggedScript, TaggedScriptList, UpdateScript, UpdateTaggedScript,
};
use sbox::models::script_tag::ScriptTag;
use sbox::models::tag::Tag;
use sbox::utils::{get_conn, DbPool};

#[get("owners/{owner_id}/scripts")]
pub async fn get_by_owner_id<'a>(
    pool: web::Data<DbPool>,
    owner_id: web::Path<i32>,
) -> Result<TaggedScriptList, ServerError<'a>> {
    /*
    TODO
        - Auth
    */
    let this_owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);

    if this_owner.id == *owner_id {
        match script::read_tagged_by_owner(&conn, &owner_id) {
            Ok(tagged_script) => Ok(tagged_script),
            Err(err) => Err(err.into()),
        }
    } else {
        // disallow requesting others scripts
        Err(ServerError::Forbidden(None))
    }
}

#[get("/scripts/{script_id}")]
pub async fn get_by_id<'a>(
    pool: web::Data<DbPool>,
    script_id: web::Path<i32>,
) -> Result<TaggedScript, ServerError<'a>> {
    /*
    TODO:
        - Auth
    */
    let owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    match script::read_tagged(&conn, &script_id) {
        Ok(tagged_script) => {
            if tagged_script.owner_id.unwrap() == owner.id {
                Ok(tagged_script)
            } else {
                Err(ServerError::Forbidden(None))
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[post("/scripts")]
pub async fn create<'a>(
    pool: web::Data<DbPool>,
    new_tagged_script: web::Json<NewTaggedScript>,
) -> Result<TaggedScript, ServerError<'a>> {
    /*
    TODO:
        - Auth
    */
    let owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    match script::create_tagged(&conn, &new_tagged_script, &owner.id) {
        Ok(tagged_script) => Ok(tagged_script),
        Err(err) => Err(err.into()),
    }
}

#[put("/scripts/{id}")]
pub async fn update<'a>(
    pool: web::Data<DbPool>,
    script_id: web::Path<i32>,
    update_script: web::Json<UpdateScript>,
) -> Result<TaggedScript, ServerError<'a>> {
    /*
    TODO:
        - Auth
    */
    let owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    match script::read_tagged(&conn, &script_id) {
        Ok(s) => {
            if s.owner_id.unwrap() == owner.id {
                match script::update(&conn, &update_script, &script_id) {
                    Ok(tagged_script) => Ok(tagged_script),
                    Err(err) => Err(err.into()),
                }
            } else {
                // Disallow updates to others scripts
                Err(ServerError::Forbidden(None))
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[delete("/scripts/{script_id}")]
pub async fn delete<'a>(
    pool: web::Data<DbPool>,
    script_id: web::Path<i32>,
) -> Result<HttpResponse, ServerError<'a>> {
    let owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    match script::read(&conn, &script_id) {
        Ok(s) => {
            if s.owner_id.unwrap() == owner.id {
                match script::delete(&conn, &script_id) {
                    Ok(_) => Ok(HttpResponse::Ok().body(Body::Empty)),
                    Err(err) => Err(err.into()),
                }
            } else {
                // Disallow delete to others scripts
                Err(ServerError::Forbidden(None))
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[derive(Debug, Deserialize)]
pub struct TagScriptQs {
    pub is_output: Option<bool>,
}

// Script tags
#[post("/scripts/{script_id}/tags/{tag_id}")]
pub async fn create_script_tag<'a>(
    pool: web::Data<DbPool>,
    web::Path((script_id, tag_id)): web::Path<(i32, i32)>,
    tag_qs: web::Query<TagScriptQs>,
) -> Result<TaggedScript, ServerError<'a>> {
    let this_owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    let is_output = match tag_qs.is_output {
        Some(val) => val,
        None => false,
    };
    match {
        // these errors should cascade - verify
        let script = script::read(&conn, &script_id)?;
        let tag = tag::read(&conn, &tag_id)?;
        Ok((script, tag))
    } {
        Ok::<(Script, Tag), diesel::result::Error>((script, tag)) => {
            if script.owner_id.unwrap() == this_owner.id
                && (tag.is_public || tag.owner_id.unwrap() == this_owner.id)
            {
                let script_tag = ScriptTag {
                    script_id: script_id.clone(),
                    tag_id: tag_id.clone(),
                    is_output,
                };
                match script_tag::create(&conn, &script_tag) {
                    Ok(tagged_script) => Ok(tagged_script),
                    Err(err) => Err(err.into()),
                }
            } else {
                // Disallow tagging others script and tagging own script with others non-public tags
                Err(ServerError::Forbidden(None))
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[delete("/scripts/{script_id}/tags/{tag_id}")]
pub async fn delete_script_tag<'a>(
    pool: web::Data<DbPool>,
    web::Path((script_id, tag_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, ServerError<'a>> {
    let this_owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    match { script::read(&conn, &script_id) } {
        Ok(script) => {
            if script.owner_id.unwrap() == this_owner.id {
                match script_tag::delete(&conn, (&script_id, &tag_id)) {
                    Ok(_) => Ok(HttpResponse::Ok().body(Body::Empty)),
                    Err(err) => Err(err.into()),
                }
            } else {
                // Disallow deletes if this owner does not own the script.
                // Not owning the tag must be allowed so public tags may be unfollowed.
                Err(ServerError::Forbidden(None))
            }
        }
        Err(err) => Err(err.into()),
    }
}
