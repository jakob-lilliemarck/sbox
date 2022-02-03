extern crate actix_web;
extern crate celery;
#[macro_use]
extern crate diesel;
extern crate r2d2;

pub mod db;
pub mod models;
pub mod routes;

use actix_web::{App, HttpServer};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(
        "postgres://sbox:dev@localhost/sbox",
    );

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool.");

    HttpServer::new(move || {
        App::new()
            .data(pool.clone())
            .service(routes::create_tag)
            .service(routes::get_tags)
    })
    .bind("localhost:8001")?
    .run()
    .await
}
