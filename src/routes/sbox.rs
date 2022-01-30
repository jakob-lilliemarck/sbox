use rocket_okapi::okapi::schemars;
use rocket_okapi::okapi::schemars::JsonSchema;
use rocket_okapi::openapi;

#[derive(Debug, FromFormField, JsonSchema)]
pub enum Language {
    Javascript,
}

#[openapi(tag = "Sbox")]
#[get("/sbox?<language>")]
pub fn get_sbox(language: Language) {
    println!("{:?}", language);
    /* Return a sbox-wasm module */
}
