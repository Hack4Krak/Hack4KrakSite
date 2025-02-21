use crate::middlewares::auth::AuthMiddleware;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

mod submit;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(submit::submit);
}

#[derive(Debug, Error)]
pub enum FlagError {
    #[error("Flag has to be in specified format: hack4KrakCTF{{...}}")]
    InvalidFlagFormat,
    #[error("This flag is not correct! Keep trying again...")]
    InvalidFlag,
    #[error("This team already submitted this flag")]
    AlreadySubmittedFlag,
}

impl ResponseError for FlagError {
    fn status_code(&self) -> StatusCode {
        match self {
            FlagError::InvalidFlagFormat | FlagError::InvalidFlag => StatusCode::BAD_REQUEST,
            FlagError::AlreadySubmittedFlag => StatusCode::CONFLICT,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
