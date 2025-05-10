use crate::models::user::Password;
use crate::models::user::validate_name_chars;
use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Json;
use actix_web::{HttpResponse, post, web};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug)]
pub struct RegisterModel {
    #[validate(length(min = 3, max = 32), custom(function = "validate_name_chars"))]
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
