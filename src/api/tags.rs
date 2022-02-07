use actix_web::{delete, get, post, put, web, HttpResponse};
use sbox::db::owner::create_owner_tag;
use sbox::db::tag::{create, read, read_by_owner};
use sbox::errors::ServerError;
use sbox::models::owner::{Follower, NewFollower};
use sbox::models::tag::TagList;
use sbox::utils::{get_conn, DbPool};

#[get("/tags")]
pub async fn tags_get_by_owner<'a>(pool: web::Data<DbPool>) -> Result<TagList, ServerError<'a>> {
    // TODO - requires owner-id to be derived from auth
    match read_by_owner(&get_conn(pool), &1) {
        Ok(tags) => Ok(tags),
        Err(err) => Err(err.into()),
    }
}

#[post("/tags")]
pub async fn tags_create<'a>(pool: web::Data<DbPool>) -> HttpResponse {
    // TODO - requires owner-id to be derived from auth
    HttpResponse::Ok().body("TODO")
}

#[put("/tags/{id}")]
pub async fn tags_update<'a>(pool: web::Data<DbPool>) -> HttpResponse {
    // TODO - auth!
    HttpResponse::Ok().body("TODO")
}

#[delete("/tags/{id}")]
pub async fn tags_delete<'a>(pool: web::Data<DbPool>) -> HttpResponse {
    // TODO - auth!
    HttpResponse::Ok().body("TODO")
}

#[post("/tags/{id}/follow")]
pub async fn tags_follow<'a>(
    pool: web::Data<DbPool>,
    tag_id: web::Path<i32>,
    new_follower: web::Json<NewFollower>,
) -> Result<Follower, ServerError<'a>> {
    // TODO - auth
    let follower = new_follower.to_follower(&*tag_id);
    let conn = get_conn(pool);
    match read(&conn, &tag_id) {
        Ok(tag) => match tag.owner_id {
            Some(owner_id) => {
                if tag.public || owner_id == follower.owner_id {
                    // Attempt to create a follower!
                    match create_owner_tag(&conn, &follower) {
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
