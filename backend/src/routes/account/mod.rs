use crate::utils::error::error_response_builder;
use actix_http::StatusCode;
use actix_web::error;
use hack4krak_macros::error_with_messages;
use utoipa_actix_web::service_config::ServiceConfig;

mod delete;
mod get_onboarding;
mod identification;
pub mod index;
mod submit_onboarding;
pub mod update;

pub use submit_onboarding::UserOnboardingSubmissionRequest;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index::index);
    cfg.service(delete::delete);
    cfg.service(update::update);
    cfg.service(update::change_password);
    cfg.service(submit_onboarding::submit_onboarding);
    cfg.service(get_onboarding::get_onboarding);
    cfg.service(identification::identification);
}

#[error_with_messages]
pub enum AccountError {
    InvalidReferralSource,
    OnboardingAlreadySubmitted,
}

impl error::ResponseError for AccountError {
    fn status_code(&self) -> StatusCode {
        match self {
            AccountError::InvalidReferralSource => StatusCode::BAD_REQUEST,
            AccountError::OnboardingAlreadySubmitted => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        error_response_builder(self)
    }
}
