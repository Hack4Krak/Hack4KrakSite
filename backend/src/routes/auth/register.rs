use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Json;
use actix_web::{post, web, HttpResponse};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct RegisterModel {
    #[validate(length(min = 3, max = 32))]
    pub name: String,
    #[validate(email)]
    pub email: String,
    pub(crate) password: String,
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
