#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

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

#[derive(Debug)]
pub enum ApiError {
    NotFound,
    InternalServerError,
}

impl fmt::Display for ApiError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            ApiError::NotFound => f.write_str("NotFound"),
            ApiError::InternalServerError => f.write_str("InternalServerError"),
        }
    }
}

impl StdError for ApiError {
    fn description(&self) -> &str {
        match *self {
            ApiError::NotFound => "Not found",
            ApiError::InternalServerError => "Internal server error",
        }
    }
}

#[derive(Debug, FromFormField, JsonSchema)]
enum Language {
    Javascript,
}

#[openapi]
#[get("/sbox?<language>")]
fn get_sbox(language: Language) {
    println!("{:?}", language);
    /* Return a sbox-wasm module */
}

#[openapi]
#[get("/source/<id>")]
fn get_source(id: i32) -> Result<Json<models::Source>, rocket::http::Status> {
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

#[openapi]
#[post("/source")]
fn new_source() {}

#[openapi]
#[put("/source/<id>")]
fn update_source(id: i32) {}

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
