use crate::utils::error::error_response_builder;
use actix_http::StatusCode;
use actix_web::error;
use hack4krak_macros::error_with_messages;
use utoipa_actix_web::service_config::ServiceConfig;

mod info;
mod label;
mod participate;
mod registration;
mod status;

pub use participate::ParticipateRequest;

pub fn config(config: &mut ServiceConfig) {
    config.service(info::info);
    config.service(status::status);
    config.service(registration::registration);
    config.service(label::label);
    config.service(participate::submit_participation);
    config.service(participate::get_participation);
}

#[error_with_messages]
pub enum EventError {
    AlreadyRegistered,
    RegistrationNotOpen,
    NotRegistered,
}

impl error::ResponseError for EventError {
    fn status_code(&self) -> StatusCode {
        match self {
            EventError::RegistrationNotOpen => StatusCode::BAD_REQUEST,
            EventError::AlreadyRegistered => StatusCode::CONFLICT,
            EventError::NotRegistered => StatusCode::NOT_FOUND,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        error_response_builder(self)
    }
}
