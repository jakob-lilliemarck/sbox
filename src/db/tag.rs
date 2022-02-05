use crate::models::tag::{NewTag, Tag, TagList};
use crate::schema::tag;

use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_owner: &NewTag) -> Result<Tag, Error> {
    diesel::insert_into(tag::table)
        .values(new_owner)
        .get_result::<Tag>(conn)
}

pub fn read_by_owner(conn: &diesel::PgConnection, id_owner: &i32) -> Result<TagList, Error> {
    use crate::schema::tag::dsl::*;
    match tag.filter(owner_id.eq(id_owner)).load::<Tag>(conn) {
        Ok(tags) => Ok(tags.into()),
        Err(err) => Err(err.into()),
    }
}
