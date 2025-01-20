use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::entities::users;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::get_tokens_http_response;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    request_body = LoginModel,
    responses(
        (status = 200, description = "User successfully logged in."),
        (status = 401, description = "Invalid credentials."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "auth"
)]
#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> Result<HttpResponse, Error> {
    let email = users::Model::verify_credentials(&app_state.database, &login_json).await?;
    let response = get_tokens_http_response(email)?;

    Ok(response)
}
