extern crate r2d2;

use actix_web::web;
use diesel::{r2d2::ConnectionManager, PgConnection};
use r2d2::PooledConnection;

pub type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

pub fn get_conn(pool: web::Data<DbPool>) -> PooledConnection<ConnectionManager<PgConnection>> {
    pool.get().expect("Could not connect to db from pool")
}
