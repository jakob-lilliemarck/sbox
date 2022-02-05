use crate::models::owner::{NewOwner, Owner};
use crate::schema::owner;

use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_owner: &NewOwner) -> Result<Owner, Error> {
    diesel::insert_into(owner::table)
        .values(new_owner)
        .get_result::<Owner>(conn)
}
