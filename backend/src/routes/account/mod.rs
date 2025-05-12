use crate::utils::error::error_response_builder;
use actix_http::StatusCode;
use actix_web::error;
use hack4krak_macros::error_with_messages;
use utoipa_actix_web::service_config::ServiceConfig;

mod delete;
mod get_personal_information;
pub mod index;
mod submit_personal_information;
pub mod update;

pub use submit_personal_information::UserPersonalInformationSubmissionRequest;

pub fn config(cfg: &mut ServiceConfig) {
    cfg.service(index::index);
    cfg.service(delete::delete);
    cfg.service(update::update);
    cfg.service(update::change_password);
    cfg.service(submit_personal_information::submit_personal_information);
    cfg.service(get_personal_information::get_personal_information);
}

#[error_with_messages]
pub enum AccountError {
    BirthYearNotInRange { min: i32, max: i32 },
    InvalidReferralSource,
}

impl error::ResponseError for AccountError {
    fn status_code(&self) -> StatusCode {
        match self {
            AccountError::BirthYearNotInRange { .. } | AccountError::InvalidReferralSource => {
                StatusCode::BAD_REQUEST
            }
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        error_response_builder(self)
    }
}
