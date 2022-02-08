use actix_web::{delete, dev::Body, get, post, put, web, HttpResponse};
use sbox::db::owner;
use sbox::db::owner_tag;
use sbox::db::script_tag;
use sbox::db::tag;
use sbox::errors::ServerError;
use sbox::models::owner::Owner;
use sbox::models::owner_tag::Follower;
use sbox::models::tag::{NewTag, Tag, TagList, UpdateTag};
use sbox::utils::{get_conn, DbPool};
/*
On tag ownership & public tags:
 - On tag deletion (orphaning), associated scripts that use the tag for tagging outputs should also be orphaned
 OR orphaned and copied to a new script for the user.

If a public tag has followers other than the owner:
 - DONE - disallow changes to is_public.
 - DONE - on delete (owner disassociation), also remove ownersip of any scripts where the tag is
 used for output, and remove the current user as the owner of the tag.
 - disallow changes or deletion of any scripts using the tag as output_tag_id.
*/

#[get("/owners/{id}/tags")]
pub async fn tags_get_by_owner<'a>(
    pool: web::Data<DbPool>,
    owner_id: web::Path<i32>,
) -> Result<TagList, ServerError<'a>> {
    /* TODO - auth*/
    let this_owner = Owner {
        id: 1,
        name: "dummy".to_string(),
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
        name: "dummy".to_string(),
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

#[put("/tags/{id}")]
pub async fn tags_update<'a>(
    pool: web::Data<DbPool>,
    update_tag: web::Json<UpdateTag>,
    tag_id: web::Path<i32>,
) -> Result<Tag, ServerError<'a>> {
    /* TODO - auth*/
    let this_owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    let t = tag::read(&conn, &tag_id)?;

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
                // If there is only one follower, and that follower is the owner of the tag, allow to make it private
                match tag::update(&conn, &update_tag, &tag_id) {
                    Ok(tag) => Ok(tag),
                    Err(err) => Err(err.into()),
                }
            } else {
                // Disallow to make orphaned tags private.
                // Disallow to make public tags private if anyone else than the owner follows them
                Err(ServerError::Forbidden(None))
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[delete("/tags/{id}")]
pub async fn tags_delete<'a>(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
) -> Result<HttpResponse, ServerError<'a>> {
    /*
    TODO
        - auth
    */
    let conn = get_conn(pool);
    let owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    match tag::read(&conn, &tag_id) {
        Ok(tag) => match tag.owner_id {
            Some(owner_id) => {
                if owner.id == owner_id {
                    // if the tag exist, this user owns the tag:
                    // 1. remove any script_tag associations where the tag is used for output
                    // 2. remove the this user as the owner of the tag
                    match {
                        script_tag::orphan_script_where_tag_is_ouput(&conn, &tag)?;
                        tag::update_owner(&conn, &tag_id, &None)?;
                        Ok(())
                    } {
                        Ok::<(), diesel::result::Error>(_) => {
                            Ok(HttpResponse::Ok().body(Body::Empty))
                        }
                        // DB update errors
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
        // Read tag errors
        Err(err) => Err(err.into()),
    }
}

#[post("/owners/{owner_id}/tags/{tag_id}")]
pub async fn create_owner_tag<'a>(
    pool: web::Data<DbPool>,
    web::Path((owner_id, tag_id)): web::Path<(i32, i32)>,
) -> Result<Follower, ServerError<'a>> {
    /*
    TODO
        - auth
        - use owner_id from path params
    */
    let conn = get_conn(pool);
    let owner = Owner {
        id: 2,
        name: "dummy".to_string(),
    };
    let follower = Follower {
        owner_id: owner.id.clone(),
        tag_id: tag_id.clone(),
    };
    match tag::read(&conn, &tag_id) {
        Ok(tag) => match tag.owner_id {
            Some(owner_id) => {
                if tag.is_public || owner_id == follower.owner_id {
                    // Attempt to create a follower!
                    match owner_tag::create_owner_tag(&conn, &follower) {
                        Ok(follower) => Ok(follower),
                        Err(err) => Err(err.into()),
                    }
                } else {
                    // Disallows following non-owned or non-public tags
                    Err(ServerError::BadRequest(Some(
                        "Cannot follow non-owned non-public tags",
                    )))
                }
            }
            // Disallows following orphaned tags
            None => Err(ServerError::BadRequest(Some("Cannot follow orphaned tags"))),
        },
        Err(err) => Err(err.into()),
    }
}

#[delete("/owners/{owner_id}/tags/{tag_id}")]
pub async fn delete_owner_tag<'a>(
    pool: web::Data<DbPool>,
    web::Path((owner_id, tag_id)): web::Path<(i32, i32)>,
) -> Result<HttpResponse, ServerError<'a>> {
    /*
    TODO
        - auth
        - use owner_id from path params
    */
    let conn = get_conn(pool);
    let owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    match tag::read(&conn, &tag_id) {
        // if tag exist continue
        Ok(_) => {
            // if tag exist create a follower instance
            let follower = Follower {
                owner_id: owner.id.clone(),
                tag_id: tag_id.clone(),
            };
            match owner_tag::follower_exist(&conn, &follower) {
                Ok(exists) => {
                    // if successfully checked if exist
                    if exists {
                        match owner_tag::delete(&conn, &follower) {
                            Ok(_) => Ok(HttpResponse::Ok().body(Body::Empty)),
                            Err(err) => Err(err.into()),
                        }
                    } else {
                        Err(ServerError::NotFound)
                    }
                }
                Err(err) => Err(err.into()),
            }
        }
        // else return err
        Err(err) => Err(err.into()),
    }
}
