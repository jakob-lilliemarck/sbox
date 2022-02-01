extern crate rocket;

use crate::models::source::{NewSource, Source, UpdateSource};
use crate::schema::source;
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;

pub fn read(
    conn: &mut diesel::PgConnection,
    source_id: &i32,
) -> Result<Source, diesel::result::Error> {
    use crate::schema::source::dsl::*;
    source.find(source_id).first::<Source>(&*conn)
}

pub fn create(conn: &mut diesel::PgConnection, new_source: &NewSource) -> Source {
    diesel::insert_into(source::table)
        .values(new_source)
        .get_result::<Source>(conn)
        .expect("Error saving new source")
}

pub fn update(conn: &mut diesel::PgConnection, source_id: &i32, update_source: &UpdateSource) {
    use crate::schema::source::dsl::*;
    let r = diesel::update(source.find(source_id))
        .set(update_source)
        .get_result::<Source>(conn);
    println!("DB RES {:?}", r);
}
