use awc::Client;
use jsonwebtoken::{decode, decode_header, jwk, DecodingKey, TokenData, Validation};
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

static TOKEN_TEST: &str = "eyJhbGciOiJSUzI1NiIsInR5cCI6IkpXVCIsImtpZCI6IlhoR3A5RENveGVfV3EtdHYzbWxUOSJ9.eyJpc3MiOiJodHRwczovL2Rldi11Mm45ZG5yOC51cy5hdXRoMC5jb20vIiwic3ViIjoiYXV0aDB8NjIwM2Q3ODQyOTg0MDcwMDc3NTJmNmU3IiwiYXVkIjpbImh0dHBzOi8vdHJhZHIuc2UiLCJodHRwczovL2Rldi11Mm45ZG5yOC51cy5hdXRoMC5jb20vdXNlcmluZm8iXSwiaWF0IjoxNjQ0NTA1NjQ3LCJleHAiOjE2NDQ1OTIwNDcsImF6cCI6IlBmNHFQMGJRdXA3VFJHd2w0VlBZbGlrY2Ywb01XV2RmIiwic2NvcGUiOiJvcGVuaWQgcHJvZmlsZSBlbWFpbCJ9.oH4bxrV9NTiAPdsCQffX4QWR1EJp3pxUNN0FcSFka-FsvkhPXeNsOPWVHbn20xnCjrPCVVau9mrqcLeBa8Mv7ZxWNl8gdtluJhIbRL-bzurP704OL8b78w8sL8NNhBjCdoFCvaDPYsWlH8bi3HqsJZkMDS-mE8_Qp2WuPr_T-Y2on10iEP4s7wRGpiso43j-yMlgnHdFAGRcQE1NG71PjPwzD1Up_Ey5OCxv3CbKg5dOAxoFX6TtGg89b4iyXt75W0b1y8ttVYpttyPOPXnmtu9qfKbus6bPBuEx-maD1-hbiOuY3WbO9M4shRmWF8KLUQGrItvqxlS-6dTCf2avVA";

async fn decodeJwk<'a>(
    token: &str,
    jwks: &jwk::JwkSet,
) -> Result<TokenData<HashMap<String, serde_json::Value>>, ServerError<'a>> {
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
                Ok(decoded_token)
            }
            _ => unreachable!("Should be RSA?"),
        }
    } else {
        return Err(ServerError::Unknown);
    }
}

pub async fn validate_token<'a>(token: &str) -> Result<bool, ServerError<'a>> {
    let authority = std::env::var("AUTHORITY").expect("AUTHORITY must be set");
    let jwks = fetch_jwks(&format!(
        "{}{}",
        authority.as_str(),
        ".well-known/jwks.json"
    ))
    .await?;
    let token_data = decodeJwk(&TOKEN_TEST, &jwks).await?;
    let user_id = token_data
        .claims
        .get("sub")
        .expect("Could not find subject on JWK")
        .as_str()
        .expect("Could not convert JWK subject to string");
    println!("{:?}", user_id); // <= This is the ID to put on the owner entity.
    Ok(true)
}
