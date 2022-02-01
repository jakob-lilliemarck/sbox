extern crate rocket;

use crate::models::source::{NewSource, Source, UpdateSource};
use crate::schema::source;
use diesel::prelude::*;
use diesel::result::Error;
use rocket_sync_db_pools::diesel;

pub fn read(conn: &mut diesel::PgConnection, source_id: &i32) -> Result<Source, Error> {
    use crate::schema::source::dsl::*;
    source.find(source_id).first::<Source>(&*conn)
}

pub fn create(conn: &mut diesel::PgConnection, new_source: &NewSource) -> Result<Source, Error> {
    diesel::insert_into(source::table)
        .values(new_source)
        .get_result::<Source>(conn)
}

pub fn update(
    conn: &mut diesel::PgConnection,
    source_id: &i32,
    update_source: &UpdateSource,
) -> Result<Source, Error> {
    use crate::schema::source::dsl::*;

    let res = diesel::update(source.find(source_id))
        .set(update_source)
        .get_result::<Source>(conn);
    res
}

pub fn delete(conn: &mut diesel::PgConnection, source_id: &i32) -> Result<usize, Error> {
    use crate::schema::source::dsl::*;
    diesel::delete(source.find(source_id)).execute(conn)
}
