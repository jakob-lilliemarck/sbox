#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use rocket::form::FromFormField;
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;

#[derive(Debug, FromFormField, JsonSchema)]
enum Language {
    Javascript,
    Python,
}

#[openapi]
#[get("/sbox?<language>")]
fn get_sbox(language: Language) {
    println!("{:?}", language);
    /*
    Lang should be an enum - make show in docs
    Return a sbox-wasm module
    */
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
#[post("/source")]
fn new_source() -> Json<models::Source> {
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
            openapi_get_routes![
                get_sbox,
                get_source,
                new_source,
                update_source,
                delete_source
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
