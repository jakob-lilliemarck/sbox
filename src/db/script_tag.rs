use crate::db::script;
use crate::models::common::IdList;
use crate::models::script::{Script, TaggedScript};
use crate::models::script_tag::{ScriptTag, UpdateScriptTag};
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
) -> Result<(Vec<i32>, Vec<i32>), Error> {
    match ScriptTag::belonging_to(script)
        .select(schema::script_tag::all_columns)
        .load::<ScriptTag>(conn)
    {
        Ok(ids) => Ok(ids
            .into_iter()
            .fold((vec![], vec![]), |mut accumulator, script_tag| {
                if script_tag.is_output {
                    accumulator.1.push(script_tag.tag_id);
                    accumulator
                } else {
                    accumulator.0.push(script_tag.tag_id);
                    accumulator
                }
            })),
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

pub fn create(conn: &diesel::PgConnection, tag_script: &ScriptTag) -> Result<TaggedScript, Error> {
    conn.transaction(|| {
        // if the relation already exist, update it else create a new relation
        let exists = script_tag_exist(&conn, &tag_script)?;
        if exists {
            update(
                &conn,
                &tag_script.to_update_script_tag(),
                tag_script.to_tuple_id(),
            )?;
        } else {
            diesel::insert_into(schema::script_tag::table)
                .values(tag_script)
                .get_result::<ScriptTag>(conn)?;
        };
        let tagged = script::read_tagged(&conn, &tag_script.script_id)?;
        Ok(tagged)
    })
}

pub fn update(
    conn: &diesel::PgConnection,
    update_script_tag: &UpdateScriptTag,
    script_tag_id: (&i32, &i32),
) -> Result<ScriptTag, Error> {
    use crate::schema::script_tag::dsl::*;
    diesel::update(script_tag.find(script_tag_id))
        .set(update_script_tag)
        .get_result::<ScriptTag>(conn)
}

pub fn create_many(
    conn: &diesel::PgConnection,
    script_tag_list: &Vec<ScriptTag>,
) -> Result<(), Error> {
    diesel::insert_into(schema::script_tag::table)
        .values(script_tag_list)
        .execute(conn)?;
    Ok(())
}

pub fn delete(conn: &diesel::PgConnection, tag_script: &ScriptTag) -> Result<(), Error> {
    use crate::schema::script_tag::dsl::*;
    match diesel::delete(script_tag.find((tag_script.script_id, tag_script.tag_id))).execute(conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn delete_all_by_script_id(conn: &diesel::PgConnection, id_script: &i32) -> Result<(), Error> {
    use crate::schema::script_tag::dsl::*;
    match diesel::delete(script_tag.filter(script_id.eq(id_script))).execute(conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
