extern crate sbox;

use crate::models::Tag;
use sbox::schema::tag;

use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

pub fn create(conn: &diesel::PgConnection, new_tag: &Tag) -> Result<Tag, Error> {
    diesel::insert_into(tag::table)
        .values(new_tag)
        .get_result::<Tag>(conn)
}

pub fn create_if_none(conn: &diesel::PgConnection, tag: &Tag) -> Result<Tag, Error> {
    match diesel::insert_into(tag::table)
        .values(tag)
        .get_result::<Tag>(conn)
    {
        Ok(tag) => Ok(tag),
        Err(err) => match err {
            Error::DatabaseError(DatabaseErrorKind::UniqueViolation, _) => read(conn, &tag.id),
            _ => Err(err),
        },
    }
}

pub fn read(conn: &diesel::PgConnection, tag_id: &String) -> Result<Tag, Error> {
    use sbox::schema::tag::dsl::*;
    tag.find(tag_id).first::<Tag>(conn)
}

pub fn delete(conn: &diesel::PgConnection, tag_id: &String) -> Option<Error> {
    use sbox::schema::tag::dsl::*;
    match diesel::delete(tag.find(tag_id)).execute(conn) {
        Ok(deleted_count) => match deleted_count {
            // Return not found err if deleted count is 0
            0 => Some(Error::NotFound),
            _ => None,
        },
        Err(err) => Some(err),
    }
}
