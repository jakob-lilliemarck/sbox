use crate::models::common::IdList;
use crate::models::script::Script;
use crate::models::script_tag::ScriptTag;
use crate::models::tag::Tag;
use crate::schema;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::select;

pub fn read_tag_by_script(conn: &diesel::PgConnection, script: &Script) -> Result<Vec<Tag>, Error> {
    ScriptTag::belonging_to(script)
        .inner_join(schema::tag::table)
        .select(schema::tag::all_columns)
        .load::<Tag>(conn)
}

pub fn read_script_by_tag(conn: &diesel::PgConnection, tag: &Tag) -> Result<Vec<Script>, Error> {
    ScriptTag::belonging_to(tag)
        .inner_join(schema::script::table)
        .select(schema::script::all_columns)
        .load::<Script>(conn)
}

pub fn read_tag_ids_by_script(
    conn: &diesel::PgConnection,
    script: &Script,
) -> Result<IdList, Error> {
    match ScriptTag::belonging_to(script)
        .select(schema::script_tag::script_id)
        .load::<i32>(conn)
    {
        Ok(ids) => Ok(IdList(ids)),
        Err(err) => Err(err),
    }
}

pub fn read_script_ids_by_tag(conn: &diesel::PgConnection, tag: &Tag) -> Result<IdList, Error> {
    match ScriptTag::belonging_to(tag)
        .select(schema::script_tag::tag_id)
        .load::<i32>(conn)
    {
        Ok(ids) => Ok(IdList(ids)),
        Err(err) => Err(err),
    }
}

pub fn script_tag_exist(
    conn: &diesel::PgConnection,
    tag_script: &ScriptTag,
) -> Result<bool, Error> {
    use crate::schema::script_tag::dsl::*;
    select(diesel::dsl::exists(
        script_tag
            .filter(tag_id.eq(tag_script.tag_id))
            .filter(script_id.eq(tag_script.script_id)),
    ))
    .get_result(conn)
}

pub fn create_script_tag(
    conn: &diesel::PgConnection,
    script_tag: &ScriptTag,
) -> Result<ScriptTag, Error> {
    diesel::insert_into(schema::script_tag::table)
        .values(script_tag)
        .get_result::<ScriptTag>(conn)
}
