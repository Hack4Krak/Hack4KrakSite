use crate::models::entities::users;
use crate::routes::auth::AuthError::InvalidEmailAddress;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::error::Error::HashPasswordFailed;
use actix_web::{post, web, HttpResponse};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHasher};
use regex::Regex;
use serde::{Deserialize, Serialize};
use utoipa::gen::serde_json::json;
use utoipa::ToSchema;

const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})$";

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterModel {
    pub name: String,
    pub email: String,
    password: String,
}

#[utoipa::path(
    request_body = RegisterModel,
    responses(
        (status = 200, description = "User successfully registered."),
        (status = 400, description = "User already registered."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "auth"
)]
#[post("/register")]
pub async fn register(
    app_state: web::Data<app_state::AppState>,
    register_json: web::Json<RegisterModel>,
) -> Result<HttpResponse, Error> {
    let regex = Regex::new(EMAIL_REGEX).unwrap();

    if !regex.is_match(&register_json.email) {
        return Err(Error::Auth(InvalidEmailAddress));
    }

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();

    let password_hash = argon2
        .hash_password(register_json.password.as_bytes(), &salt)
        .map_err(HashPasswordFailed)?
        .to_string();

    users::Model::create_with_password(&app_state.database, password_hash, &register_json).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}
