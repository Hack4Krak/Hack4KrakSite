use crate::models::entities::users;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::get_default_tokens;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
    let email = users::Model::verify_credentials(&app_state.database, &login_json).await?;
    let tokens = get_default_tokens(email)?;

    Ok(HttpResponse::Ok().json(tokens))
}
