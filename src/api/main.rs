extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate r2d2;
extern crate sbox;

use actix_web::{App, HttpServer};

//pub mod data;
pub mod owners;
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
            .service(owners::owner_create)
            .service(tags::tags_get_by_owner)
            .service(scripts::scripts_create)
            .service(scripts::scripts_get_own)
            .service(scripts::scripts_get_id)
            .service(scripts::scripts_delete)
    })
    .bind("localhost:8000")?
    .run()
    .await
}
