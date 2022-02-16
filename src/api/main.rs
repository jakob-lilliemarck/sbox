extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate awc;
extern crate futures;
extern crate jsonwebtoken;
extern crate r2d2;
extern crate sbox;

//pub mod data;
//pub mod auth;
//pub mod auth2;
pub mod owners;
pub mod scripts;
pub mod tags;

use actix_web::{
    dev::Extensions,
    dev::{Service, ServiceRequest},
    middleware, web,
    web::ReqData,
    App, Error, FromRequest, HttpMessage, HttpRequest, HttpServer,
};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use actix_web_httpauth::middleware::HttpAuthentication;
use dotenv;
use futures::future::FutureExt;

/*
AUTH - todo
- If an authenticated user does not exist in the owner-table - create a new owner entity.
- Remove owner endpoints!
- Better errorhandling
- Break up auth code and cleanup!

DB connection - todo
- Get rid of dependency on rocket_db_pool lib - use something intended for diesel/actix instead.

API - todo
- Look through figma & check against spec
- Remove redundant functionality
- Consider implementing db methods on model-sValidationtructs fo flow becomes:
    - deserialize json to struct
    - use a method on the struct to perform DB operations -> return updated struct
    - serialize JSON & return to client app
- Consider how business-logic rules should work
    - As methods on the structs?
    - How to minimize DB reads while keeping readability?
- Can app.serice calls be pre-grouped and exposed in their respecive file? Would be neat.
*/
/*
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
*/

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let manager = diesel::r2d2::ConnectionManager::<diesel::PgConnection>::new(
        "postgres://sbox:dev@localhost/sbox",
    );

    let pool = r2d2::Pool::builder()
        .build(manager)
        .expect("Failed to create db pool.");

    HttpServer::new(move || {
        //let auth = HttpAuthentication::bearer(auth2::validator);
        App::new()
            .data(pool.clone())
            .wrap(sbox::middleware::auth::Auth {
                jwks_expire: std::time::Duration::new(60, 0),
                jwks_url: "https://dev-u2n9dnr8.us.auth0.com/.well-known/jwks.json",
            })
            //.wrap(middleware::Logger::default())
            //.wrap(auth)
            /*.wrap_fn(|req, srv| {
                req.extensions_mut().insert(Test {
                    val: "hej".to_string(),
                });
                srv.call(req).map(|res| res)
            })*/
            // owners
            .service(owners::owner_create)
            .service(owners::create_owner_tag)
            .service(owners::delete_owner_tag)
            // tags
            .service(tags::tags_get_by_owner)
            .service(tags::tags_create)
            //.service(tags::tags_update)
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
