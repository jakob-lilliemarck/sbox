use actix_web::{delete, dev::Body, get, post, put, web, HttpResponse};
use sbox::db::owner_tag;
use sbox::db::script_tag;
use sbox::db::tag;
use sbox::errors::ServerError;
use sbox::models::owner::Owner;
use sbox::models::tag::{NewTag, Tag, TagList, UpdateTag};
use sbox::utils::{get_conn, DbPool};

/*
On tag ownership & public tags:
 - On tag deletion (orphaning), associated scripts that use the tag for tagging outputs should also be orphaned
 OR orphaned and copied to a new script for the user. <= CONSIDER THIS!

If a public tag has followers other than the owner:
 - DONE - disallow changes to is_public.
 - DONE - disallow delete of any tags used for output.
 - disallow changes or deletion of any scripts using the tag as output_tag_id.
*/

#[get("/owners/{id}/tags")]
pub async fn tags_get_by_owner<'a>(
    pool: web::Data<DbPool>,
    owner_id: web::Path<i32>,
    test: web::ReqData<&str>,
) -> Result<TagList, ServerError<'a>> {
    println!("TEST FROM MIDDLEWARE: {:?}", test);

    /* TODO - auth*/
    let this_owner = Owner {
        id: 1,
        external_id: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    if this_owner.id != *owner_id {
        match owner_tag::read_public_tag_by_owner(&conn, &this_owner) {
            Ok(tags) => Ok(TagList(tags)),
            Err(err) => Err(err.into()),
        }
    } else {
        match owner_tag::read_tag_by_owner(&conn, &this_owner) {
            Ok(tags) => Ok(TagList(tags)),
            Err(err) => Err(err.into()),
        }
    }
}

#[post("/tags")]
pub async fn tags_create<'a>(
    pool: web::Data<DbPool>,
    new_tag: web::Json<NewTag>,
) -> Result<Tag, ServerError<'a>> {
    /* TODO - auth*/
    let owner = Owner {
        id: 1,
        external_id: "dummy".to_string(),
    };
    let conn = get_conn(pool);

    match tag::read_by_value_and_owner(&conn, &new_tag.value, &owner.id) {
        Ok(tag) => Ok(tag),
        Err(err) => match err {
            // If a tag with that value does not already exist for this user, try to create it.
            diesel::result::Error::NotFound => match tag::create(&conn, &new_tag, &owner.id) {
                Ok(tag) => Ok(tag),
                Err(err) => Err(err.into()),
            },
            // If there is another error, cast it into a ServerError.
            _ => Err(err.into()),
        },
    }
}
/*
#[put("/tags/{id}")]
pub async fn tags_update<'a>(
    pool: web::Data<DbPool>,
    update_tag: web::Json<UpdateTag>,
    tag_id: web::Path<i32>,
) -> Result<Tag, ServerError<'a>> {
    /* TODO - auth*/
    let this_owner = Owner {
        id: 1,
        external_id: "dummy".to_string(),
    };
    let conn = get_conn(pool);

    let owner_list = {
        let tag = tag::read(&conn, &tag_id)?;
        let owner_list = owner_tag::read_owner_by_tag(&conn, &tag)?;
        Ok(owner_list)
    };

    match owner_list {
        Ok::<Vec<Owner>, diesel::result::Error>(owner_list) => {
            if owner_list.len() == 1
                && owner_list
                    .into_iter()
                    .all(|owner| owner.id == this_owner.id)
            {
                match tag::update(&conn, &update_tag, &tag_id) {
                    Ok(tag) => Ok(tag),
                    Err(err) => Err(err.into()),
                }
            } else {
                // Disallow updates to orphaned tags.
                // Disallow updates to tag followed by others than the owner.
                Err(ServerError::Forbidden(None))
            }
        }
        Err(err) => Err(err.into()),
    }
}
*/
#[delete("/tags/{id}")]
pub async fn tags_delete<'a>(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
) -> Result<HttpResponse, ServerError<'a>> {
    /* TODO - auth */
    let conn = get_conn(pool);
    let owner = Owner {
        id: 1,
        external_id: "dummy".to_string(),
    };
    match tag::read(&conn, &tag_id) {
        Ok(tag) => match tag.owner_id {
            Some(owner_id) => {
                if owner.id == owner_id {
                    match script_tag::read_script_by_tag_is_output(&conn, &tag) {
                        Ok(scripts) => {
                            // TODO - allow if all scripts are orphaned
                            if scripts.len() == 0 {
                                tag::update_owner(&conn, &tag_id, &None)?;
                                Ok(HttpResponse::Ok().body(Body::Empty))
                            } else {
                                // Disallow  updates of tags used for script output.
                                Err(ServerError::Forbidden(Some("Tags used for script output may not be deleted, you need to unassign the tag or orphan the script first.")))
                            }
                        }
                        Err(err) => Err(err.into()),
                    }
                } else {
                    // Disallow updates of non-owned tags.
                    Err(ServerError::Forbidden(None))
                }
            }
            // Disallow updates on orphaned tags
            None => Err(ServerError::Forbidden(None)),
        },
        Err(err) => Err(err.into()),
    }
}
