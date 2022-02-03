extern crate rocket;
extern crate sbox;

use crate::models::script::{NewScript, Script, UpdateScript};
use sbox::schema::script;

use diesel::prelude::*;
use diesel::result::Error;
use rocket_sync_db_pools::diesel;

pub fn read(conn: &mut diesel::PgConnection, script_id: &i32) -> Result<Script, Error> {
    use sbox::schema::script::dsl::*;
    script.find(script_id).first::<Script>(&*conn)
}

pub fn create(conn: &mut diesel::PgConnection, new_source: &NewScript) -> Result<Script, Error> {
    diesel::insert_into(script::table)
        .values(new_source)
        .get_result::<Script>(conn)
}

pub fn update(
    conn: &mut diesel::PgConnection,
    source_id: &i32,
    update_source: &UpdateScript,
) -> Result<Script, Error> {
    use sbox::schema::script::dsl::*;

    let res = diesel::update(script.find(source_id))
        .set(update_source)
        .get_result::<Script>(conn);
    res
}

pub fn delete(conn: &mut diesel::PgConnection, source_id: &i32) -> Result<usize, Error> {
    use sbox::schema::script::dsl::*;
    diesel::delete(script.find(source_id)).execute(conn)
}
