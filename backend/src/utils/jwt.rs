use std::future;

use crate::services::env::EnvConfig;
use crate::utils::error::Error;
use crate::utils::error::Error::InvalidJsonWebToken;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use chrono::{TimeDelta, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct JwtClaims {
    #[serde(rename = "sub")]
    pub id: Uuid,
    pub email: String,
    #[serde(rename = "exp")]
    pub expiration_time: usize,
    #[serde(rename = "iat")]
    pub issued_at: usize,
}

impl FromRequest for JwtClaims {
    type Error = Error;

    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.extensions().get::<JwtClaims>() {
            Some(data) => future::ready(Ok(data.clone())),
            None => future::ready(Err(Error::MissingExtension {
                name: "jwt_claims".to_string(),
            })),
        }
    }
}

pub fn encode_jwt(id: Uuid, email: String, expire: TimeDelta) -> Result<String, Error> {
    let now = Utc::now();

    let claims = JwtClaims {
        expiration_time: (now + expire).timestamp() as usize,
        issued_at: now.timestamp() as usize,
        id,
        email,
    };

    let secret = &EnvConfig::get().jwt_secret;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| InvalidJsonWebToken)
}

pub fn decode_jwt(jwt: &str) -> Result<TokenData<JwtClaims>, jsonwebtoken::errors::Error> {
    let secret = &EnvConfig::get().jwt_secret;
    let claim_data: Result<TokenData<JwtClaims>, jsonwebtoken::errors::Error> = decode(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    claim_data
}
