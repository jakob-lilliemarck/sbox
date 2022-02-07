use crate::models::script::{Script, ScriptTag};
use crate::models::tag::{NewTag, Tag, TagList};
use crate::schema::tag;

use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_owner: &NewTag) -> Result<Tag, Error> {
    diesel::insert_into(tag::table)
        .values(new_owner)
        .get_result::<Tag>(conn)
}

pub fn read(conn: &diesel::PgConnection, tag_id: &i32) -> Result<Tag, Error> {
    use crate::schema::tag::dsl::*;
    tag.find(tag_id).first::<Tag>(conn)
}

pub fn create_many(conn: &diesel::PgConnection, new_tags: &Vec<NewTag>) -> Result<Vec<Tag>, Error> {
    conn.transaction(|| {
        new_tags
            .iter()
            .map(|new_tag| {
                // Check for tags with equal value - values must be unique for one owner
                match {
                    use crate::schema::tag::dsl::*;
                    tag.filter(owner_id.eq(&new_tag.owner_id))
                        .filter(value.eq(&new_tag.value))
                        .first::<Tag>(conn)
                } {
                    // if the tag exists, return the results
                    Ok(tag) => Ok(tag),
                    Err(err) => match err {
                        // if the tag was not found, create it and return results
                        Error::NotFound => diesel::insert_into(tag::table)
                            .values(new_tag)
                            .get_result::<Tag>(conn),
                        // pass on any other errors
                        _ => Err(err),
                    },
                }
            })
            .collect()
    })
}

pub fn read_by_owner(conn: &diesel::PgConnection, id_owner: &i32) -> Result<TagList, Error> {
    use crate::schema::tag::dsl::*;
    match tag.filter(owner_id.eq(id_owner)).load::<Tag>(conn) {
        Ok(tags) => Ok(tags.into()),
        Err(err) => Err(err.into()),
    }
}

pub fn read_by_script(conn: &diesel::PgConnection, script: &Script) -> Result<Vec<Tag>, Error> {
    ScriptTag::belonging_to(script)
        .inner_join(tag::table)
        .select(tag::all_columns)
        .load::<Tag>(conn)
}
