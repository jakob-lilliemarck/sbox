use crate::models::owner::{Follower, NewOwner, Owner};
use crate::schema::{owner, owner_tag, tag};
use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_owner: &NewOwner) -> Result<Owner, Error> {
    diesel::insert_into(owner::table)
        .values(new_owner)
        .get_result::<Owner>(conn)
}

pub fn create_owner_tag(
    conn: &diesel::PgConnection,
    follower: &Follower,
) -> Result<Follower, Error> {
    diesel::insert_into(owner_tag::table)
        .values(follower)
        .get_result::<Follower>(conn)
}
