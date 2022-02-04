extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate sbox;

use actix_web::{App, HttpServer};

pub mod inputs;
pub mod outputs;
pub mod scripts;
pub mod tags;

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
            // inputs
            .service(inputs::create_input)
            .service(inputs::get_input)
            // tags
            .service(tags::create_tag)
            .service(tags::get_tags)
            .service(tags::delete_tag)
    })
    .bind("localhost:8001")?
    .run()
    .await
}
