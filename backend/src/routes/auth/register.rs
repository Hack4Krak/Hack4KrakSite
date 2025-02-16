use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterModel {
    pub name: String,
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
    model: web::Json<RegisterModel>,
) -> Result<HttpResponse, Error> {
    AuthService::register_with_password(&app_state, model.into_inner()).await
}
