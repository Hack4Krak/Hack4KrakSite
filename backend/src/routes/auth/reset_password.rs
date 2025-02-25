use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, patch, post};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RequestResetPasswordModel {
    pub email: String,
}

#[utoipa::path(
    request_body = RequestResetPasswordModel,
    responses(
        (status = 200, description = "Password reset email sent."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "auth"
)]
#[post("/request_reset_password")]
pub async fn request_reset_password(
    app_state: Data<app_state::AppState>,
    model: Json<RequestResetPasswordModel>,
) -> Result<HttpResponse, Error> {
    AuthService::request_password_reset(&app_state, model.into_inner().email).await?;

    Ok(HttpResponse::Ok().body("Password reset email sent."))
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ResetPasswordModel {
    pub code: Uuid,
    pub(crate) new_password: String,
}

#[utoipa::path(
    request_body = ResetPasswordModel,
    responses(
        (status = 200, description = "Password successfully reset."),
        (status = 400, description = "Invalid reset code."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "auth"
)]
#[patch("/reset_password")]
pub async fn reset_password(
    app_state: Data<app_state::AppState>,
    model: Json<ResetPasswordModel>,
) -> Result<HttpResponse, Error> {
    AuthService::reset_password(&app_state, model.into_inner()).await?;

    Ok(HttpResponse::Ok().body("Password successfully reset."))
}
