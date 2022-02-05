use crate::models::inputs::{Input, NewInput, NewTaggedInput};
use crate::models::tags::Tag;
use crate::schema::*;

use diesel::prelude::*;
use diesel::result::Error;
use itertools::Itertools;

impl NewInput {
    pub fn create(self, conn: &diesel::PgConnection) -> Result<Input, Error> {
        diesel::insert_into(input::table)
            .values(self)
            .get_result::<Input>(conn)
    }
}

impl NewTaggedInput {
    pub fn create(self, conn: &diesel::PgConnection) {
        /*
        conn.transaction(|| {
            match NewInput::from_new_tagged_input(&self).create(conn) {
                Ok(new_input) => Ok(new_input),
                Err(_err) => Err(Error::RollbackTransaction),
            }
            .expect("Err creating input");

            let new_tags: Vec<Tag> = self
                .tags
                .into_iter()
                .map(|tag_string| {
                    let tag = Tag { id: tag_string };
                    tag
                })
                .collect();

            let created = diesel::insert_into(tag::table)
                .values(&new_tags)
                .get_result::<Tag>(conn);

            created
        })
        .expect("Error creating TaggedInput");
        */
    }
}

pub fn create_tagged_input(conn: &diesel::PgConnection, new_tagged_input: &NewTaggedInput) {
    let NewTaggedInput { data, tags } = new_tagged_input;
    let input = NewInput { data: data.clone() };
    let tags: Vec<Tag> = tags.into_iter().map(|id| Tag { id: id.clone() }).collect();

    let res = conn.transaction(|| {
        tags.iter()
            .map(|t| {
                println!("TAG: {:?}", t);
                let tt = diesel::insert_into(tag::table)
                    .values(t)
                    .get_result::<Tag>(conn);
                println!("CREATED: {:?}", tt);
                tt
            })
            .fold_ok(vec![], |mut a, b| {
                a.push(b);
                a
            })
    });
    println!("RES: {:?}", res);
}

pub fn create(conn: &diesel::PgConnection, new_input: &Input) -> Result<Input, Error> {
    diesel::insert_into(input::table)
        .values(new_input)
        .get_result::<Input>(conn)
}
/*
pub fn tag(
    conn: &diesel::PgConnection,
    input_tag: &InputTag,
) -> Result<InputTag, Error> {
    diesel::insert_into(input_tag::table)
        .values(input_tag)
        .get_result::<InputTag>(conn)
}
*/
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
