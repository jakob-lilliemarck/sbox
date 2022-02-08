use crate::db::owner_tag;
use crate::errors::ServerError;
use crate::models::owner::Owner;
use crate::models::tag::Tag;
use diesel::result::Error;

impl Tag {
    pub fn if_not_followed<F>(
        &self,
        owner: &Owner,
        conn: &diesel::PgConnection,
        f: F,
    ) -> Result<Tag, ServerError>
    where
        F: FnOnce() -> Result<Tag, Error>,
    {
        match owner_tag::read_owner_by_tag(&conn, &self) {
            Ok(owner_list) => {
                if owner_list.len() == 1 && owner_list.into_iter().all(|o| o.id == owner.id) {
                    // If there is only one follower, and that follower is the owner of the tag, allow to make it private
                    let t = f();
                    match t {
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
}
