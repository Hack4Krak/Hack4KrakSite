use crate::utils::error::error_response_builder;
use actix_http::StatusCode;
use actix_web::error;
use hack4krak_macros::error_with_messages;
use utoipa_actix_web::service_config::ServiceConfig;

mod delete;
mod get_onboarding;
pub mod index;
mod submit_onboarding;
pub mod update;
mod verification;

pub use submit_onboarding::UserOnboardingSubmissionRequest;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index::index);
    cfg.service(delete::delete);
    cfg.service(update::update);
    cfg.service(update::change_password);
    cfg.service(submit_onboarding::submit_onboarding);
    cfg.service(get_onboarding::get_onboarding);
    cfg.service(verification::verification);
}

#[error_with_messages]
pub enum AccountError {
    OnboardingAlreadySubmitted,
}

impl error::ResponseError for AccountError {
    fn status_code(&self) -> StatusCode {
        match self {
            AccountError::OnboardingAlreadySubmitted => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        error_response_builder(self)
    }
}
