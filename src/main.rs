#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use rocket::serde::json::Json;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[openapi]
#[post("/source")]
fn new_source() -> Json<models::Source> {
    Json(models::Source {
        id: 1,
        lang: "lang",
        src: "src",
    })
}

#[openapi]
#[get("/source/<id>")]
fn get_source(id: i32) -> Json<models::Source> {
    Json(models::Source {
        id: 1,
        lang: "lang",
        src: "src",
    })
}

#[openapi]
#[put("/source/<id>")]
fn update_source(id: i32) -> Json<models::Source> {
    Json(models::Source {
        id: 1,
        lang: "lang",
        src: "src",
    })
}

#[openapi]
#[delete("/source/<id>")]
fn delete_source(id: i32) {}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            openapi_get_routes![new_source, get_source, update_source, delete_source],
        )
        .mount(
            "/docs",
            make_swagger_ui(&SwaggerUIConfig {
                url: "../openapi.json".to_owned(),
                ..Default::default()
            }),
        )
}
