use crate::db::tag::create_many;
use crate::models::script::{NewScript, Script};
use crate::models::tag::{NewTag, Tag};
use crate::schema::script;

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
        Ok((script, tags))
    })
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

pub fn read_by_owner(conn: &diesel::PgConnection, id_owner: &i32) -> Result<Vec<Script>, Error> {
    use crate::schema::script::dsl::*;
    match script.filter(owner_id.eq(id_owner)).load::<Script>(conn) {
        Ok(scripts) => Ok(scripts.into()),
        Err(err) => Err(err.into()),
    }
}
