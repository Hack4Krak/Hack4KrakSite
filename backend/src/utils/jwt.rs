use std::future;

use crate::utils::cookies::{create_cookie, ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE};
use crate::utils::env::Config;
use crate::utils::error::Error;
use crate::utils::error::Error::InvalidJsonWebToken;
use actix_web::{FromRequest, HttpMessage, HttpResponse, HttpResponseBuilder};
use chrono::{Duration, TimeDelta, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, TokenData, Validation};
use sea_orm::prelude::Uuid;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct Claims {
    #[serde(rename = "sub")]
    pub id: Uuid,
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

pub fn encode_jwt(id: Uuid, email: String, expire: TimeDelta) -> Result<String, Error> {
    let now = Utc::now();

    let claims = Claims {
        expiration_time: (now + expire).timestamp() as usize,
        issued_at: now.timestamp() as usize,
        id,
        email,
    };

    let secret = &Config::get().jwt_secret;

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_ref()),
    )
    .map_err(|_| InvalidJsonWebToken)
}

pub fn decode_jwt(jwt: &str) -> Result<TokenData<Claims>, jsonwebtoken::errors::Error> {
    let secret = &Config::get().jwt_secret;
    let claim_data: Result<TokenData<Claims>, jsonwebtoken::errors::Error> = decode(
        jwt,
        &DecodingKey::from_secret(secret.as_ref()),
        &Validation::default(),
    );

    claim_data
}

pub fn append_tokens_as_cookies(
    uuid: Uuid,
    email: String,
    http_response: &mut HttpResponseBuilder,
) -> Result<(), Error> {
    let access_token = encode_jwt(uuid, email.clone(), Duration::minutes(10))?;
    let refresh_token = encode_jwt(uuid, email, Duration::days(14))?;

    let refresh_cookie = create_cookie(
        REFRESH_TOKEN_COOKIE,
        &refresh_token,
        Some(actix_web::cookie::time::Duration::days(14)),
    );
    let access_cookie = create_cookie(ACCESS_TOKEN_COOKIE, &access_token, None);

    http_response.append_header(("Set-Cookie", refresh_cookie));
    http_response.append_header(("Set-Cookie", access_cookie));

    Ok(())
}

pub fn get_tokens_http_response(uuid: Uuid, email: String) -> Result<HttpResponse, Error> {
    let mut response = HttpResponse::Ok();
    append_tokens_as_cookies(uuid, email, &mut response)?;
    Ok(response.finish())
}
