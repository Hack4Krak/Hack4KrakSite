use crate::models::user::Password;
use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Json;
use actix_web::{HttpResponse, post, web};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::{Validate, ValidationError};

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug)]
pub struct RegisterModel {
    #[validate(length(min = 3, max = 32), custom(function = "is_latin_extended"))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    #[validate(length(min = 8, max = 32))]
    pub password: Password,
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
    Validated(model): Validated<Json<RegisterModel>>,
) -> Result<HttpResponse, Error> {
    AuthService::register_with_password(&app_state, model.into_inner()).await
}
fn is_latin_extended(username: &str) -> Result<(), ValidationError> {
    if username.chars().all(|c| {
        c.is_ascii_alphanumeric()
            || ('\u{00C0}'..='\u{024F}').contains(&c)
            || c.is_ascii_punctuation()
    }) {
        return Ok(());
    }
    Err(ValidationError::new("invalid_username_chars"))
}
