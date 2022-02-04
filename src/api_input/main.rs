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
            // todo - attach json error handling here..? se JsonConfig: https://docs.rs/actix-web/3.0.0-beta.3/actix_web/web/struct.JsonConfig.html
            .service(routes::create_input)
            .service(routes::get_input)
    })
    .bind("localhost:8001")?
    .run()
    .await
}
