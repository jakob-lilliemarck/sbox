use crate::db::tag::create_many;
use crate::models::script::{NewScript, Script, ScriptList};
use crate::models::tag::{NewTag, Tag};
use crate::schema::{script, tag};

use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_script: &NewScript) -> Result<Script, Error> {
    diesel::insert_into(script::table)
        .values(new_script)
        .get_result::<Script>(conn)
}

pub fn create_tagged(
    conn: &diesel::PgConnection,
    new_script: &NewScript,
    new_tags: &Vec<NewTag>,
) -> Result<(Script, Vec<Tag>), Error> {
    conn.transaction(|| {
        let script = create(&conn, &new_script)?;
        let tags = create_many(&conn, &new_tags)?;
        Ok((script, tags))
    })
}
/*
pub fn create_tagged_script(
    conn: &diesel::PgConnection,
    new_script_and_tags: &NewScriptAndTags,
) -> Result<Script, Error> {
    conn.transaction(|| {
        let res = vec![];
        let NewScriptAndTags(new_script, new_tags) = new_script_and_tags;

        let mut script = diesel::insert_into(script::table)
            .values(new_script)
            .get_result::<Script>(conn);

        res.push(script);

        let mut tags = new_tags
            .into_iter()
            .map(|t| {
                diesel::insert_into(tag::table)
                    .values(t)
                    .get_result::<Tag>(conn)
            })
            .collect::<Result<Tag, Error>>();
        // append to result vector
        res.append(&mut tags);

        let s = Script {
            id: 1,
            owner_id: 1,
            source: "Hej".to_string(),
        };

        Ok(s)
    })
}
*/
pub fn read(conn: &diesel::PgConnection, script_id: &i32) -> Result<Script, Error> {
    use crate::schema::script::dsl::*;
    script.find(script_id).first::<Script>(conn)
}

pub fn read_by_owner(conn: &diesel::PgConnection, id_owner: &i32) -> Result<ScriptList, Error> {
    use crate::schema::script::dsl::*;
    match script.filter(owner_id.eq(id_owner)).load::<Script>(conn) {
        Ok(scripts) => Ok(scripts.into()),
        Err(err) => Err(err.into()),
    }
}
