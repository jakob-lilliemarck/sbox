#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate celery;

use rocket_okapi::{openapi_get_routes, swagger_ui::*};

pub mod db;
pub mod models;
pub mod routes;

#[launch]
async fn rocket() -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .attach(db::Conn::fairing())
        .mount(
            "/",
            openapi_get_routes![
                routes::script::create_script,
                routes::script::read_script,
                routes::script::update_script,
                routes::script::delete_script
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
