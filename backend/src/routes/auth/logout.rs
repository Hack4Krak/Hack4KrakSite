use crate::services::auth::AuthService;
use crate::utils::error::Error;
use actix_web::{HttpResponse, post};

#[utoipa::path(
    responses(
        (status = 200, description = "User successfully logged out.")
    ),
    tag = "auth"
)]
#[post("/logout")]
pub async fn logout() -> Result<HttpResponse, Error> {
    Ok(AuthService::reset_cookies_response())
}
