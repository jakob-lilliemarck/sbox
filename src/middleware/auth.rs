use crate::db::owner::read;
use crate::utils::get_conn;
use crate::utils::DbPool;
use actix_web::dev::{Service, ServiceRequest, ServiceResponse, Transform};
use actix_web::{web, Error};
use futures::future::{ok, Ready};
use futures::Future;
use jsonwebtoken::jwk::JwkSet;
use std::pin::Pin;
use std::task::{Context, Poll};

// There are two steps in middleware processing.
// 1. Middleware initialization, middleware factory gets called with
//    next service in chain as parameter.
// 2. Middleware's call method gets called with normal request.
pub struct Auth<'a> {
    pub jwks_expire: std::time::Duration,
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
            jwks_url: self.jwks_url,
        })
    }
}

pub struct AuthMiddleware<'a, S> {
    service: S,
    jwks_expire: std::time::Duration,
    jwks_url: &'a str,
}

impl<'a, S> AuthMiddleware<'a, S> {
    pub fn fetch_jwks(&self) {
        println!("Fetch JWKS from auth provider...")
    }
    pub fn authorize(&self) {
        println!("Check authorization...")
    }

    pub fn authenticate(&self) {
        println!("Authenticate & append Owner to context...")
    }
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
        let pool = req.app_data::<DbPool>().expect("No db pool found!");
        let conn = pool.get().expect("Could not get conn");
        let test = read(&conn, &1);
        println!("OWNER? {:?}", test);
        self.fetch_jwks();
        self.authorize();
        self.authenticate();
        // request
        let fut = self.service.call(req);
        Box::pin(async move {
            let res = fut.await?;
            // response
            Ok(res)
        })
    }
}
