#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod guards;
pub mod models;
pub mod schema;

use rocket::form::FromFormField;
use rocket::http::Status;
use rocket::serde::json::Json;
use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::{openapi, openapi_get_routes, swagger_ui::*};

use diesel::prelude::*;
use diesel::result::Error;
use dotenv::dotenv;
use std::error::Error as StdError;
use std::{env, fmt};

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[derive(Debug, FromFormField, JsonSchema)]
enum Language {
    Javascript,
}

#[openapi(tag = "Sbox")]
#[get("/sbox?<language>")]
fn get_sbox(language: Language) {
    println!("{:?}", language);
    /* Return a sbox-wasm module */
}

#[openapi(tag = "Source")]
#[get("/source/<id>")]
fn get_source(id: i32, _db: guards::DB) -> Result<Json<models::Source>, rocket::http::Status> {
    use self::schema::source::dsl::*;
    let connection = establish_connection();
    let s = source.find(id).first::<models::Source>(&connection);
    match s {
        Ok(s) => Ok(Json(s)),
        Err(err) => match err {
            _ => Err(Status::NotFound),
        },
    }
}

#[openapi(tag = "Source")]
#[post("/source")]
fn new_source() {}

#[openapi(tag = "Source")]
#[put("/source/<id>")]
fn update_source(id: i32) {}

#[openapi(tag = "Source")]
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
