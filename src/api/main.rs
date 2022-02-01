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
fn rocket() -> rocket::Rocket<rocket::Build> {
    /*
    TODO:
    - Make a fairing for runtime instantiation.
    - Expose a function that takes an async closure as argument, and pass it as a request guard to the routes
    - Consider error-handling - what could go wrong?
    */
    let mut runtime = tokio::runtime::Runtime::new().unwrap();

    let res = match runtime.block_on(async {
        let my_app = sbox::celery::create_app();

        my_app.send_task(sbox::celery::add::new(1, 2)).await
    }) {
        Ok(x) => Ok(x),
        Err(_) => Err(println!("Listener failure")),
    };
    /*ENDOF TODO*/

    rocket::build()
        .attach(db::Conn::fairing())
        .mount(
            "/",
            openapi_get_routes![
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
