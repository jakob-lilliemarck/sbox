use crate::db;
use crate::models::common::IdList;
use crate::models::script::{
    NewScript, NewTaggedScript, Script, TaggedScript, TaggedScriptList, UpdateScript,
};
use crate::models::script_tag::ScriptTag;
use crate::schema::script;

use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_script: &NewScript) -> Result<Script, Error> {
    diesel::insert_into(script::table)
        .values(new_script)
        .get_result::<Script>(conn)
}

pub fn read(conn: &diesel::PgConnection, script_id: &i32) -> Result<Script, Error> {
    use crate::schema::script::dsl::*;
    script.find(script_id).get_result(conn)
}

pub fn update(
    conn: &diesel::PgConnection,
    update_script: &UpdateScript,
    script_id: &i32,
) -> Result<Script, Error> {
    use crate::schema::script::dsl::*;
    diesel::update(script.find(script_id))
        .set(update_script)
        .get_result::<Script>(conn)
}

pub fn delete(conn: &diesel::PgConnection, script_id: &i32) -> Result<(), Error> {
    // TODO!
    // Make sure all script_tag relations are deleted!!!
    use crate::schema::script::dsl::*;
    match diesel::delete(script.find(script_id)).execute(conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}

pub fn create_tagged(
    conn: &diesel::PgConnection,
    new_tagged_script: &NewTaggedScript,
    owner_id: &i32,
) -> Result<TaggedScript, Error> {
    let NewTaggedScript { source, tag_ids } = new_tagged_script;
    let new_script = NewScript {
        owner_id: owner_id.clone(),
        source: source.clone(),
    };
    conn.transaction(|| {
        let script = create(&conn, &new_script)?;
        let script_tag_list = tag_ids
            .into_iter()
            .map(|tag_id| ScriptTag {
                tag_id: tag_id.clone(),
                script_id: script.id.clone(),
            })
            .collect();
        db::script_tag::create_many(&conn, &script_tag_list)?;
        Ok((script, IdList(tag_ids.clone())).into())
    })
}

pub fn read_tagged(conn: &diesel::PgConnection, script_id: &i32) -> Result<TaggedScript, Error> {
    conn.transaction(|| {
        let script = read(&conn, script_id)?;
        let tag_ids = db::script_tag::read_tag_ids_by_script(&conn, &script)?;
        Ok((script, tag_ids).into())
    })
}

pub fn read_tagged_by_owner(
    conn: &diesel::PgConnection,
    id_owner: &i32,
) -> Result<TaggedScriptList, Error> {
    conn.transaction(|| {
        use crate::schema::script::dsl::*;
        let owner_script_ids = script
            .filter(owner_id.eq(id_owner))
            .select(id)
            .load::<i32>(conn)?;
        let tagged = owner_script_ids
            .iter()
            .map(|id_script| read_tagged(&conn, &id_script))
            .collect::<Result<Vec<TaggedScript>, Error>>()?;
        Ok(TaggedScriptList(tagged))
    })
}
