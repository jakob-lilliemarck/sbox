use crate::models::tags::Tag;
use crate::schema::tag;

use diesel::prelude::*;
use diesel::result::{DatabaseErrorKind, Error};

impl Tag {
    pub fn create(self, conn: &diesel::PgConnection) -> Result<Tag, Error> {
        diesel::insert_into(tag::table)
            .values(self)
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

    pub fn read() {}
}

// Old below

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
    use crate::schema::tag::dsl::*;
    tag.find(tag_id).first::<Tag>(conn)
}

pub fn delete(conn: &diesel::PgConnection, tag_id: &String) -> Option<Error> {
    use crate::schema::tag::dsl::*;
    match diesel::delete(tag.find(tag_id)).execute(conn) {
        Ok(deleted_count) => match deleted_count {
            // Return not found err if deleted count is 0
            0 => Some(Error::NotFound),
            _ => None,
        },
        Err(err) => Some(err),
    }
}
