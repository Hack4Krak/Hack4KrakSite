use crate::services::auth::AuthService;
use crate::services::env::EnvConfig;
use crate::utils::app_state;
use crate::utils::common_responses;
use crate::utils::common_responses::create_temporary_redirect_response;
use crate::utils::error::Error;
use actix_web::{HttpResponse, get, web};

#[utoipa::path(
    responses(
        (status = 200, description = "Email successfully confirmed. Redirecting..."),
        (status = 307, description = "Invalid confirmation code."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "auth"
)]
#[get("/confirm/{confirmation_code}")]
pub async fn confirm_email(
    app_state: web::Data<app_state::AppState>,
    confirmation_code: web::Path<String>,
) -> Result<HttpResponse, Error> {
    match AuthService::confirm_email(&app_state, confirmation_code.into_inner()).await {
        Ok(()) => {
            let url = EnvConfig::get()
                .frontend_url
                .join("/login?redirect_from_confirmation=true")?;

            let mut response = common_responses::create_redirect_response(url)?;

            Ok(response.body("Email successfully confirmed. Redirecting..."))
        }
        Err(error) => {
            let url = EnvConfig::get().frontend_url.join("/panel")?;
            Ok(create_temporary_redirect_response(url, error)?.finish())
        }
    }
}
