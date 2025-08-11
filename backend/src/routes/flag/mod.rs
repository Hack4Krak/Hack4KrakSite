use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use hack4krak_macros::error_with_messages;

mod submit;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(submit::submit);
}

#[error_with_messages]
pub enum FlagError {
    InvalidFlagFormat,
    InvalidFlag,
    AlreadySubmittedFlag,
    TeamNotConfirmed,
}

impl ResponseError for FlagError {
    fn status_code(&self) -> StatusCode {
        match self {
            FlagError::InvalidFlagFormat | FlagError::InvalidFlag => StatusCode::BAD_REQUEST,
            FlagError::AlreadySubmittedFlag => StatusCode::CONFLICT,
            FlagError::TeamNotConfirmed => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
