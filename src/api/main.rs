extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate futures;
extern crate r2d2;
extern crate reqwest;
extern crate sbox;

use actix_web::{dev::ServiceRequest, middleware, App, Error, HttpServer};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;

//pub mod data;
pub mod auth;
pub mod owners;
pub mod scripts;
pub mod tags;

async fn validator<'a>(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);
    match auth::validate_token(credentials.token()).await {
        Ok(res) => {
            if res == true {
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(
        "postgres://sbox:dev@localhost/sbox",
    );

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool.");

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(validator);
        App::new()
            .wrap(middleware::Logger::default())
            .wrap(auth)
            .data(pool.clone())
            // owners
            .service(owners::owner_create)
            .service(owners::create_owner_tag)
            .service(owners::delete_owner_tag)
            // tags
            .service(tags::tags_get_by_owner)
            .service(tags::tags_create)
            .service(tags::tags_update)
            .service(tags::tags_delete)
            // scripts
            .service(scripts::create)
            .service(scripts::get_by_id)
            .service(scripts::get_by_owner_id)
            .service(scripts::update)
            .service(scripts::delete)
            .service(scripts::create_script_tag)
        //.service(scripts::scripts_get_own)
        //.service(scripts::scripts_get_id)
        //.service(scripts::scripts_delete)
    })
    .bind("localhost:8001")?
    .run()
    .await
}
