use crate::db::Conn;
use rocket_okapi::okapi::schemars;
use rocket_okapi::openapi;

#[derive(Debug, FromFormField, schemars::JsonSchema)]
pub enum Language {
    Javascript,
}

#[openapi(tag = "Sbox")]
#[get("/sbox?<language>")]
pub fn get_sbox(_conn: Conn, language: Language) {
    println!("{:?}", language);
    /* Return a sbox-wasm module */
}
