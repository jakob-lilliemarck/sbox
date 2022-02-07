use crate::db::owner_tag::create_owner_tag;
use crate::models::owner_tag::Follower;
use crate::models::script::Script;
use crate::models::script_tag::ScriptTag;
use crate::models::tag::{NewTag, Tag, UpdateTag};
use crate::schema::tag;

use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_tag: &NewTag, owner_id: &i32) -> Result<Tag, Error> {
    conn.transaction(|| {
        let tag = diesel::insert_into(tag::table)
            .values((new_tag, tag::dsl::owner_id.eq(owner_id)))
            .get_result::<Tag>(conn)?;

        let follower = Follower {
            owner_id: owner_id.clone(),
            tag_id: tag.id.clone(),
        };
        create_owner_tag(&conn, &follower)?;
        Ok(tag)
    })
}

pub fn read(conn: &diesel::PgConnection, tag_id: &i32) -> Result<Tag, Error> {
    use crate::schema::tag::dsl::*;
    tag.find(tag_id).first::<Tag>(conn)
}

pub fn read_by_value_and_owner<'a>(
    conn: &diesel::PgConnection,
    val: &'a str,
    id_owner: &i32,
) -> Result<Tag, Error> {
    use crate::schema::tag::dsl::*;
    tag.filter(owner_id.eq(id_owner))
        .filter(value.eq(val))
        .first::<Tag>(conn)
}

pub fn update(
    conn: &diesel::PgConnection,
    update_tag: &UpdateTag,
    tag_id: &i32,
) -> Result<Tag, Error> {
    use crate::schema::tag::dsl::*;
    diesel::update(tag.find(tag_id))
        .set(update_tag)
        .get_result::<Tag>(conn)
}

// consider using generic update() instead - but make sure to now allow owner-changes from clients
pub fn update_owner(
    conn: &diesel::PgConnection,
    tag_id: &i32,
    id_owner: &Option<i32>,
) -> Result<Tag, Error> {
    use crate::schema::tag::dsl::*;
    diesel::update(tag.find(tag_id))
        .set(owner_id.eq(id_owner))
        .get_result::<Tag>(conn)
}

pub fn read_by_script(conn: &diesel::PgConnection, script: &Script) -> Result<Vec<Tag>, Error> {
    ScriptTag::belonging_to(script)
        .inner_join(tag::table)
        .select(tag::all_columns)
        .load::<Tag>(conn)
}
