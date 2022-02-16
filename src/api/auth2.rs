use actix_web::{dev::ServiceRequest, Error, HttpMessage};
use actix_web_httpauth::extractors::bearer::{BearerAuth, Config};
use actix_web_httpauth::extractors::AuthenticationError;
use awc::Client;
use jsonwebtoken::{
    decode, decode_header,
    jwk::{AlgorithmParameters, JwkSet},
    DecodingKey, TokenData, Validation,
};
use sbox::db::owner::read_from_external_id;
use sbox::errors::ServerError;
use sbox::models::owner::Owner;
use std::collections::HashMap;

pub async fn validator<'a>(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, Error> {
    let config = req
        .app_data::<Config>()
        .map(|data| data.clone())
        .unwrap_or_else(Default::default);

    let token = credentials.token();
    match validate_token(token).await {
        Ok((res, jwks)) => {
            if res == true {
                let token_data = decode_jwk(&token, &jwks).await?;
                let user_id = token_data
                    .claims
                    .get("sub")
                    .expect("Could not find subject on JWK")
                    .as_str()
                    .expect("Could not convert JWK subject to string");
                println!("{:?}", user_id);
                Ok(req)
            } else {
                Err(AuthenticationError::from(config).into())
            }
        }
        Err(_) => Err(AuthenticationError::from(config).into()),
    }
}

pub async fn validate_token<'a>(token: &str) -> Result<(bool, JwkSet), ServerError<'a>> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await?;
    // TODO - actually validate!
    Ok((true, jwks))
}

async fn fetch_jwks(uri: &str) -> Result<JwkSet, Box<dyn std::error::Error>> {
    let client = Client::default();
    let mut res = client.get(uri).send().await?;
    let body = res.body().await?;
    let resp_data = std::str::from_utf8(&body).unwrap();
    let jwks: JwkSet = serde_json::from_str(resp_data).unwrap();
    return Ok(jwks);
}

async fn decode_jwk<'a>(
    token: &str,
    jwks: &JwkSet,
) -> Result<TokenData<HashMap<String, serde_json::Value>>, ServerError<'a>> {
    let header = decode_header(&token).expect("err decode header - TODO: into ServerError");
    let kid = match header.kid {
        Some(kid) => kid,
        None => return Err(ServerError::Unknown),
    };
    if let Some(j) = jwks.find(&kid) {
        match j.algorithm {
            AlgorithmParameters::RSA(ref rsa) => {
                let decoding_key = DecodingKey::from_rsa_components(&rsa.n, &rsa.e).unwrap(); // errors?
                let mut validation = Validation::new(j.common.algorithm.unwrap());
                validation.validate_exp = false;
                let decoded_token = decode::<HashMap<String, serde_json::Value>>(
                    &token,
                    &decoding_key,
                    &validation,
                )
                .unwrap();
                Ok(decoded_token)
            }
            _ => unreachable!("Should be RSA?"),
        }
    } else {
        return Err(ServerError::Unknown);
    }
}

fn load_or_create_owner<'a>(external_id: &str) -> Result<Owner, ServerError<'a>> {
    match read_from_external_id(conn, id_external)
}
