#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;

use rocket_okapi::{openapi_get_routes, swagger_ui::*};

pub mod db;
pub mod models;
pub mod routes;
pub mod schema;

#[launch]
fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(db::Conn::fairing())
        .mount(
            "/",
            openapi_get_routes![
                routes::sbox::get_sbox,
                routes::source::create_source,
                routes::source::read_source,
                routes::source::update_source,
                routes::source::delete_source
            ],
        )
        .mount(
            "/",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
