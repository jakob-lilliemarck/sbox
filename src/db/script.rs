use crate::db::tag::{create_many, read_by_script};
use crate::models::script::{NewScript, Script, ScriptTag, TaggedScript, UpdateTaggedScript};
use crate::models::tag::{NewTag, Tag};
use crate::schema::{script, script_tag};

use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_script: &NewScript) -> Result<Script, Error> {
    diesel::insert_into(script::table)
        .values(new_script)
        .get_result::<Script>(conn)
}

pub fn create_script_tag(
    conn: &diesel::PgConnection,
    script_tag: &ScriptTag,
) -> Result<ScriptTag, Error> {
    diesel::insert_into(script_tag::table)
        .values(script_tag)
        .get_result::<ScriptTag>(conn)
}

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

pub fn read_tagged(conn: &diesel::PgConnection, script_id: &i32) -> Result<TaggedScript, Error> {
    use crate::schema::script::dsl::*;
    match script.find(script_id).first::<Script>(conn) {
        Ok(s) => {
            let tags = read_by_script(&conn, &s).expect("Error reading script tags");
            Ok((s, tags).into())
        }
        Err(err) => Err(err),
    }
}
/*
pub fn update_tagged(
    conn: &diesel::PgConnection,
    tagged_script: &UpdateTaggedScript,
    script_id: &i32,
) {
    use crate::schema::script::dsl::*;
    let s = script.find(script_id).first::<Script>(conn);
    let s_t = ScriptTag::belonging_to(&s)
        .select(tag::all_columns)
        .load::<Tag>(conn);
    /*
    TODO

    1. split NewTagScript in Script and Vec<Tag>
    2. create_if_none() for all tags, store returns in a var "tags".
    3. filter() current ScriptTags
    4. compare ScriptTags with ids in "tags" variable, delete any relations that's not present in the tags variable

    Q:
        -
    */
}
*/
pub fn delete_tagged(conn: &diesel::PgConnection, id_script: &i32) -> Result<(), Error> {
    conn.transaction(|| {
        use crate::schema::script::dsl::script;
        use crate::schema::script_tag::dsl::{script_id, script_tag};
        diesel::delete(script_tag.filter(script_id.eq(id_script))).execute(conn)?;
        diesel::delete(script.find(id_script)).execute(conn)?;
        Ok(())
    })
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
