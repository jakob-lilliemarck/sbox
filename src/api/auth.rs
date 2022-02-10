use awc::Client;
use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, Validation};
use sbox::errors::ServerError;
use std::collections::HashMap;

/*
Docs at:
https://github.com/Keats/jsonwebtoken/blob/master/examples/auth0.rs#L22
*/
async fn fetch_jwks(uri: &str) -> Result<jwk::JwkSet, Box<dyn std::error::Error>> {
    let client = Client::default();
    let mut res = client.get(uri).send().await?;
    let body = res.body().await?;
    let resp_data = std::str::from_utf8(&body).unwrap();
    let jwks: jwk::JwkSet = serde_json::from_str(resp_data).unwrap();
    return Ok(jwks);
}

pub async fn validate_token<'a>(token: &str) -> Result<bool, ServerError<'a>> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await?;
    let header = decode_header(&token).expect("err decode header - TODO: into ServerError");
    let kid = match header.kid {
        Some(kid) => kid,
        None => return Err(ServerError::Unknown),
    };
    if let Some(j) = jwks.find(&kid) {
        match j.algorithm {
            jwk::AlgorithmParameters::RSA(ref rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap(); // errors?
                let mut validation = Validation::new(j.common.algorithm.unwrap());
                validation.validate_exp = false;
                let decoded_token = decode::<HashMap<String, serde_json::Value>>(
                    &token,
                    &decoding_key,
                    &validation,
                )
                .unwrap();
                println!("DECODED: {:?}", decoded_token);
            }
            _ => unreachable!("Should be RSA?"),
        }
    } else {
        return Err(ServerError::Unknown);
    }
    Ok(true)
}
