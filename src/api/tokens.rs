use actix_web::{delete, dev::Body, post, web, HttpResponse};
use sbox::db::owner::create;
use sbox::db::owner_tag;
use sbox::db::tag;
use sbox::errors::ServerError;
use sbox::utils::{get_conn, DbPool};

#[post("/tokens")]
async pub fn get_token() {}

#[delete("/tokens")]
async pub fn revoke_token() {}
