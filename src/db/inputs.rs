use crate::models::inputs::Input;
use crate::models::tags::Tag;
use crate::schema::*;

use diesel::prelude::*;
use diesel::result::Error;

pub fn create(conn: &diesel::PgConnection, new_input: &Input) -> Result<Input, Error> {
    diesel::insert_into(input::table)
        .values(new_input)
        .get_result::<Input>(conn)
}

pub fn read(conn: &diesel::PgConnection, input_id: &i32) -> Result<Input, Error> {
    use crate::schema::input::dsl::*;
    let test = input
        .inner_join(input_tag::table.inner_join(tag::table))
        .filter(id.eq(input_id))
        .select(tag::all_columns)
        .load::<Tag>(conn)?;
    println!("TEST: {:?}", test);
    input.find(input_id).first::<Input>(conn)
}
