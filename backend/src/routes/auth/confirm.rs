use crate::services::auth::AuthService;
use crate::services::env::EnvConfig;
use crate::utils::app_state;
use crate::utils::common_responses;
use crate::utils::error::Error;
use actix_web::{get, web, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Email successfully confirmed. Redirecting..."),
        (status = 401, description = "Invalid confirmation code."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "auth"
)]
#[get("/confirm/{confirmation_code}")]
pub async fn confirm_email(
    app_state: web::Data<app_state::AppState>,
    confirmation_code: web::Path<String>,
) -> Result<HttpResponse, Error> {
    AuthService::confirm_email(&app_state, confirmation_code.into_inner()).await?;

    let mut response = common_responses::create_redirect_response(
        EnvConfig::get().email_confirm_redirect_url.clone(),
    );

    Ok(response.body("Email successfully confirmed. Redirecting..."))
}
