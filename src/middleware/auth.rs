use crate::db::owner::read;
use crate::utils::get_conn;
use crate::utils::DbPool;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Error};
use awc::Client;
use futures::future::{ok, Ready};
use futures::Future;
use jsonwebtoken::jwk::JwkSet;
use std::pin::Pin;
use std::str::from_utf8;
use std::task::{Context, Poll};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth<'a> {
    pub jwks_expire: Duration,
    pub jwks_timestamp: SystemTime,
    pub jwks_url: &'a str,
}

impl<'a> Auth<'a> {
    fn from_env() {
        // Factory function returning an Auth struct configured from env vars
    }
}

// Middleware factory is `Transform` trait from actix-service crate
// `S` - type of the next service
// `B` - type of response's body
impl<'a, S, B> Transform<S> for Auth<'a>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<'a, S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ok(AuthMiddleware {
            service,
            jwks_expire: self.jwks_expire,
            jwks_timestamp: self.jwks_timestamp,
            jwks_url: self.jwks_url,
        })
    }
}

pub struct AuthMiddleware<'a, S> {
    service: S,
    jwks_expire: Duration,
    jwks_timestamp: SystemTime,
    jwks_url: &'a str,
}

impl<'a, S> AuthMiddleware<'a, S> {
    pub async fn fetch_jwks(&self) -> Result<JwkSet, Error> {
        // TODO - impl from errors or handle them in match cases.
        let client = Client::default();
        let mut response = client.get(self.jwks_url).send().await?;
        let body = response.body().await?;
        let response_json = from_utf8(&body).expect("Err converting jwks to String");
        let jwks: JwkSet =
            serde_json::from_str(&response_json).expect("Err deserializing jwks json");
        Ok(jwks)
    }

    pub fn check_jwks(&mut self) {
        match SystemTime::now().duration_since(self.jwks_timestamp) {
            Ok(dur) => {
                if dur > self.jwks_expire {
                    println!("fetch new jwks");
                    self.jwks_timestamp = SystemTime::now();
                } else {
                    println!("use existing jwks");
                }
            }
            Err(_) => panic!("SystemTime before UNIX EPOCH!"),
        };
    }
    /*
    pub fn authorize(&self, token: &'a str) -> Result<bool, Error> {
        println!("Check authorization...")
    }

    pub fn authenticate(&self, 'a &str) -> Result<Owner, Error> {
        println!("Authenticate & append Owner to context...")
    }

    pub fn check_jwt(token: &'a str) -> Result<bool, Error> {
        match Self::authorize(&token) {
            Ok(authorized) => match authenticate {
                Ok(authenticated) => Ok(true),
                Err(err) => {
                    // log error
                    Err(err)
                }
            }
            Err(err) => {
                // log error
                Err(err)
            }
        }
        Ok(true)
    }
    */
}

impl<'a, S, B> Service for AuthMiddleware<'a, S>
where
    S: Service<Request = ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Request = ServiceRequest;
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(cx)
    }

    fn call(&mut self, req: ServiceRequest) -> Self::Future {
        //let pool = req.app_data::<DbPool>().expect("No db pool found!");
        //let conn = pool.get().expect("Could not get conn");
        //let test = read(&conn, &1);
        //println!("OWNER? {:?}", test);
        self.check_jwks();
        //self.fetch_jwks();
        //self.authorize();
        //self.authenticate();
        // request
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            // response
            Ok(res)
        })
    }
}
