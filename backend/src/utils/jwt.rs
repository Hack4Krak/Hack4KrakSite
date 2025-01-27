use crate::routes::auth::TokensResponse;
use crate::utils::env::CONFIG;
use crate::utils::error::Error;
use actix_web::{FromRequest, HttpMessage};
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use serde::{Deserialize, Serialize};
use std::future;

#[derive(Serialize, Deserialize, Clone)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub email: String,
    #[serde(rename = "exp")]
    pub expiration_time: usize,
    #[serde(rename = "iat")]
    pub issued_at: usize,
}

impl FromRequest for Claims {
    type Error = actix_web::Error;

    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(
        req: &actix_web::HttpRequest,
        _payload: &mut actix_web::dev::Payload,
    ) -> future::Ready<Result<Claims, actix_web::Error>> {
        match req.extensions().get::<Claims>() {
            Some(claim) => future::ready(Ok(claim.clone())),
            None => future::ready(Err(actix_web::error::ErrorBadRequest("Bad Claims"))),
        }
    }
}

pub fn encode_jwt(email: String, expire: TimeDelta) -> Result<String, Error> {
    let now = Utc::now();

    let claims = Claims {
        expiration_time: (now + expire).timestamp() as usize,
        issued_at: now.timestamp() as usize,
        email,
    };

    let secret = &CONFIG.jwt_secret;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| Error::InvalidJsonWebToken)
}

pub fn decode_jwt(jwt: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = &CONFIG.jwt_secret;
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    claim_data
}

pub fn get_default_tokens(email: String) -> Result<TokensResponse, Error> {
    let access_token = encode_jwt(email.clone(), Duration::minutes(10))?;
    let refresh_token = encode_jwt(email, Duration::days(14))?;

    Ok(TokensResponse {
        access_token,
        refresh_token,
    })
}
