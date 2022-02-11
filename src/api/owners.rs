use actix_web::{delete, dev::Body, post, web, HttpResponse};
use sbox::db::owner::create;
use sbox::db::owner_tag;
use sbox::db::tag;
use sbox::errors::ServerError;
use sbox::models::owner::{NewOwner, Owner};
use sbox::models::owner_tag::Follower;
use sbox::utils::{get_conn, DbPool};

#[post("/owners")]
pub async fn owner_create<'a>(
    pool: web::Data<DbPool>,
    new_owner: web::Json<NewOwner>,
) -> Result<Owner, ServerError<'a>> {
    /*
    AUTH
    1. Password and email in POST body
    2. Store password as a hash (use secret_key to encode)
    3. On basic auth, check provided pass encodes to the same hash, return jwt
    4. On token-protected routes, check token & expiration on owner model
    */
    match create(&get_conn(pool), &new_owner) {
        Ok(owner) => Ok(owner),
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
        external_id: "dummy".to_string(),
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
        external_id: "dummy",
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
