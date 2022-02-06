use crate::db::tag::{create_many, read_by_script};
use crate::models::script::{NewScript, Script, ScriptTag, TaggedScript};
use crate::models::tag::{NewTag, Tag};
use crate::schema::{script, script_tag};

use diesel::prelude::*;
use diesel::result::Error;

pub fn create_tagged(
    conn: &diesel::PgConnection,
    new_script: &NewScript,
    new_tags: &Vec<NewTag>,
) -> Result<(Script, Vec<Tag>), Error> {
    conn.transaction(|| {
        let script = create(&conn, &new_script)?;
        let tags = create_many(&conn, &new_tags)?;
        // create script_tag relations - No unqiue-error since script is allways new record.
        for tag in tags.iter() {
            let script_tag = ScriptTag {
                script_id: script.id,
                tag_id: tag.id,
            };
            create_script_tag(conn, &script_tag)?;
        }
        Ok((script, tags))
    })
}

pub fn create_script_tag(
    conn: &diesel::PgConnection,
    script_tag: &ScriptTag,
) -> Result<ScriptTag, Error> {
    diesel::insert_into(script_tag::table)
        .values(script_tag)
        .get_result::<ScriptTag>(conn)
}

pub fn create(conn: &diesel::PgConnection, new_script: &NewScript) -> Result<Script, Error> {
    diesel::insert_into(script::table)
        .values(new_script)
        .get_result::<Script>(conn)
}

pub fn read(conn: &diesel::PgConnection, script_id: &i32) -> Result<Script, Error> {
    use crate::schema::script::dsl::*;
    script.find(script_id).first::<Script>(conn)
}

pub fn read_by_owner(
    conn: &diesel::PgConnection,
    id_owner: &i32,
) -> Result<Vec<TaggedScript>, Error> {
    use crate::schema::script::dsl::*;
    match script.filter(owner_id.eq(id_owner)).load::<Script>(conn) {
        Ok(scripts) => {
            Ok(scripts
                .into_iter()
                .map(|s| {
                    // Get tags for each script here & cast to TaggedScript!
                    let tags = read_by_script(&conn, &s).expect("Error reading script tags"); // Unhandled error case? Ideally, errors reading tags should interrupt iteration
                    (s, tags).into()
                })
                .collect::<Vec<TaggedScript>>())
        }
        // Propagate errors reading script
        Err(err) => Err(err),
    }
}
