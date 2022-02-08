use crate::models::common::IdList;
use crate::models::owner::Owner;
use crate::models::owner_tag::Follower;
use crate::models::tag::Tag;
use crate::schema;

use diesel::prelude::*;
use diesel::result::Error;
use diesel::select;

pub fn read_tag_by_owner(conn: &diesel::PgConnection, owner: &Owner) -> Result<Vec<Tag>, Error> {
    Follower::belonging_to(owner)
        .inner_join(schema::tag::table)
        .select(schema::tag::all_columns)
        .load::<Tag>(conn)
}

pub fn read_public_tag_by_owner(
    conn: &diesel::PgConnection,
    owner: &Owner,
) -> Result<Vec<Tag>, Error> {
    Follower::belonging_to(owner)
        .inner_join(schema::tag::table)
        .select(schema::tag::all_columns)
        .filter(schema::tag::is_public.eq(true))
        .load::<Tag>(conn)
}

pub fn read_owner_by_tag(conn: &diesel::PgConnection, tag: &Tag) -> Result<Vec<Owner>, Error> {
    Follower::belonging_to(tag)
        .inner_join(schema::owner::table)
        .select(schema::owner::all_columns)
        .load::<Owner>(conn)
}

pub fn read_tag_ids_by_owner(conn: &diesel::PgConnection, owner: &Owner) -> Result<IdList, Error> {
    match Follower::belonging_to(owner)
        .select(schema::owner_tag::owner_id)
        .load::<i32>(conn)
    {
        Ok(ids) => Ok(IdList(ids)),
        Err(err) => Err(err),
    }
}

pub fn read_owner_ids_by_tag(conn: &diesel::PgConnection, tag: &Tag) -> Result<IdList, Error> {
    match Follower::belonging_to(tag)
        .select(schema::owner_tag::tag_id)
        .load::<i32>(conn)
    {
        Ok(ids) => Ok(IdList(ids)),
        Err(err) => Err(err),
    }
}

pub fn follower_exist(conn: &diesel::PgConnection, follower: &Follower) -> Result<bool, Error> {
    use crate::schema::owner_tag::dsl::*;
    select(diesel::dsl::exists(
        owner_tag
            .filter(tag_id.eq(follower.tag_id))
            .filter(owner_id.eq(follower.owner_id)),
    ))
    .get_result(conn)
}

pub fn create_owner_tag(
    conn: &diesel::PgConnection,
    follower: &Follower,
) -> Result<Follower, Error> {
    diesel::insert_into(schema::owner_tag::table)
        .values(follower)
        .get_result::<Follower>(conn)
}

pub fn delete(conn: &diesel::PgConnection, follower: &Follower) -> Result<(), Error> {
    use crate::schema::owner_tag::dsl::*;
    match diesel::delete(owner_tag.find((follower.owner_id, follower.tag_id))).execute(conn) {
        Ok(_) => Ok(()),
        Err(err) => Err(err),
    }
}
