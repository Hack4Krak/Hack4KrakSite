use crate::utils::error::Error;
use actix_web::{post, web, HttpResponse};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use chrono::Duration;
use regex::Regex;

use crate::models::entities::users;
use crate::utils::app_state;
use crate::utils::jwt::{decode_jwt, encode_jwt};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const EMAIL_REGEX: &str = r#"(?m)(?:[a-z0-9!#$%&'*+/=?^_`{|}~-]+(?:\.[a-z0-9!#$%&'*+/=?^_`{|}~-]+)*|"(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21\x23-\x5b\x5d-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])*")@(?:(?:[a-z0-9](?:[a-z0-9-]*[a-z0-9])?\.)+[a-z0-9](?:[a-z0-9-]*[a-z0-9])?|\[(?:(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?)\.){3}(?:25[0-5]|2[0-4][0-9]|[01]?[0-9][0-9]?|[a-z0-9-]*[a-z0-9]:(?:[\x01-\x08\x0b\x0c\x0e-\x1f\x21-\x5a\x53-\x7f]|\\[\x01-\x09\x0b\x0c\x0e-\x7f])+)\])"#;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterModel {
    pub name: String,
    pub email: String,
    password: String,
}

#[utoipa::path(
    post,
    path = "/register",
    request_body = RegisterModel,
    responses(
        (status = 200, description = "User successfully registered."),
        (status = 400, description = "User already registered."),
        (status = 500, description = "Internal server error.")
    )
)]
#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>,
) -> Result<HttpResponse, Error> {
    let regex = Regex::new(EMAIL_REGEX).unwrap();

    if !regex.is_match(&register_json.email) {
        return Err(Error::InvalidEmailAddress);
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(register_json.password.as_bytes(), &salt)
        .map_err(Error::HashPasswordFailed)?
        .to_string();

    users::Model::create_with_password(&app_state.database, password_hash, &register_json).await?;

    Ok(HttpResponse::Ok().body(""))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TokensResponse {
    pub access_token: String,
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/login",
    request_body = LoginModel,
    responses(
        (status = 200, description = "User successfully logged in."),
        (status = 401, description = "Invalid credentials."),
        (status = 500, description = "Internal server error.")
    )
)]
#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> Result<HttpResponse, Error> {
    let Some(email) = users::Model::verify_credentials(&app_state.database, &login_json).await?
    else {
        return Err(Error::InvalidCredentials);
    };

    let access_token = encode_jwt(email.clone(), Duration::minutes(10))?;
    let refresh_token = encode_jwt(email, Duration::days(14))?;

    Ok(HttpResponse::Ok().json(TokensResponse {
        access_token,
        refresh_token,
    }))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshToken,
    responses(
        (status = 200, description = "New tokens received"),
        (status = 401, description = "Invalid credentials."),
    )
)]
#[post("/refresh")]
pub async fn refresh(data: web::Json<RefreshToken>) -> Result<HttpResponse, Error> {
    let claim = decode_jwt(&data.refresh_token).map_err(|_| Error::Unauthorized)?;
    let email = claim.claims.email;

    let access_token = encode_jwt(email.clone(), Duration::minutes(10))?;
    let refresh_token = encode_jwt(email, Duration::days(14))?;

    Ok(HttpResponse::Ok().json(TokensResponse {
        access_token,
        refresh_token,
    }))
}

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(register);
    cfg.service(login);
    cfg.service(refresh);
}
