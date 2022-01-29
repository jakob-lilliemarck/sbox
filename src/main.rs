#[macro_use]
extern crate rocket;
#[macro_use]
extern crate diesel;
extern crate dotenv;

pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use rocket_dyn_templates::Template;
use serde::Serialize;
use std::env;

pub fn establish_connection() -> PgConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    PgConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

#[derive(Serialize, Debug)]
struct Endpoint<'a> {
    method: &'a str,
}

#[derive(Serialize, Debug)]
struct Docs<'a> {
    routes: Vec<&'a Endpoint<'a>>,
}

/* ROUTES --- --- ---*/
#[get("/")]
fn index() -> Template {
    let a = Endpoint { method: "a" };

    let context = Docs { routes: vec![&a] };
    Template::render("docs", &context)
}

#[post("/source")]
fn new_source() -> &'static str {
    "TODO - new source!"
}

#[put("/source/<id>")]
fn update_source(id: i32) -> &'static str {
    "TODO - update source!"
}

/* Main --- --- ---*/
#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![index])
        .attach(Template::fairing())
}
