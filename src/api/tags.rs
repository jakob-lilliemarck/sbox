use actix_web::{delete, dev::Body, get, post, put, web, HttpResponse};
use sbox::db::owner;
use sbox::db::owner_tag;
use sbox::db::tag;
use sbox::errors::ServerError;
use sbox::models::owner::Owner;
use sbox::models::owner_tag::Follower;
use sbox::models::tag::{NewTag, Tag, TagList, UpdateTag};
use sbox::utils::{get_conn, DbPool};

#[get("/owners/{id}/tags")]
pub async fn tags_get_by_owner<'a>(
    pool: web::Data<DbPool>,
    owner_id: web::Path<i32>,
) -> Result<TagList, ServerError<'a>> {
    /*
    TODO
    - requires owner-id to be derived from auth
    - if requested as owner, should return all followed tags
    - if requested as non-owner, should return all public followed tags
    */
    let user = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);

    match owner::read(&conn, &owner_id) {
        Ok(owner) => {
            if user.id == *owner_id {
                // requesting own tags
                match owner_tag::read_tag_by_owner(&conn, &owner) {
                    Ok(tags) => Ok(TagList(tags)),
                    Err(err) => Err(err.into()),
                }
            } else {
                match owner_tag::read_public_tag_by_owner(&conn, &owner) {
                    Ok(tags) => Ok(TagList(tags)),
                    Err(err) => Err(err.into()),
                }
            }
        }
        Err(err) => Err(err.into()),
    }
}

#[post("/tags")]
pub async fn tags_create<'a>(
    pool: web::Data<DbPool>,
    new_tag: web::Json<NewTag>,
) -> Result<Tag, ServerError<'a>> {
    /*
    TODO
    - requires owner-id to be derived from auth
    */
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
    /*
    TODO
    - requires owner-id to be derived from auth
    */
    let owner = Owner {
        id: 1,
        name: "dummy".to_string(),
    };
    let conn = get_conn(pool);
    match tag::read(&conn, &tag_id) {
        Ok(tag) => match tag.owner_id {
            Some(owner_id) => {
                if owner.id == owner_id {
                    // Update the tag
                    match tag::update(&conn, &update_tag, &tag_id) {
                        Ok(tag) => Ok(tag),
                        Err(err) => Err(err.into()),
                    }
                } else {
                    // Disallow updates on others' tags
                    Err(ServerError::Forbidden(None))
                }
            }
            // Disallow update to orphaned tags
            None => Err(ServerError::Forbidden(None)),
        },
        // Error reading the tag
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
    - requires owner-id to be derived from auth
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
                    // if the tag exist, and this user owns the tag, disassociate the user from the tag.
                    match tag::update_owner(&conn, &tag_id, &None) {
                        Ok(_) => Ok(HttpResponse::Ok().body(Body::Empty)),
                        Err(err) => Err(err.into()),
                    }
                } else {
                    Err(ServerError::Forbidden(None))
                }
            }
            // Forbid updates on orphaned tags
            None => Err(ServerError::Forbidden(None)),
        },
        // Error reading the tag
        Err(err) => Err(err.into()),
    }
}

#[post("/tags/{id}/follow")]
pub async fn tags_follow<'a>(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
) -> Result<Follower, ServerError<'a>> {
    /*
    TODO
    - requires owner-id to be derived from auth
    */
    let conn = get_conn(pool);
    let owner = Owner {
        id: 1,
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

#[delete("/tags/{id}/follow")]
pub async fn tags_delete_follow<'a>(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
) -> Result<HttpResponse, ServerError<'a>> {
    /*
    TODO
    - requires owner-id to be derived from auth
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
