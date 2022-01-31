extern crate rocket;

use crate::models::source::{NewSource, Source};
use diesel::prelude::*;
use rocket_sync_db_pools::diesel;

pub fn read(
    conn: &mut diesel::PgConnection,
    source_id: &i32,
) -> Result<Source, diesel::result::Error> {
    use crate::schema::source::dsl::*;
    source.find(source_id).first::<Source>(&*conn)
}

pub fn create<'a>(conn: &mut diesel::PgConnection, new_source: &'a NewSource) {
    use crate::schema::source;
    println!("IN DB: {:?}", new_source);
    diesel::insert_into(source::table)
        .values(new_source)
        .get_result(&*conn)
        .expect("Error saving new source")
}
