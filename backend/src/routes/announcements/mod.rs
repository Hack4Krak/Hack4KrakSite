use actix_http::StatusCode;
use actix_web::{HttpResponse, error};
use hack4krak_macros::error_with_messages;

pub mod latest;
pub mod list;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(latest::latest).service(list::list);
}

#[error_with_messages]
pub enum AnnouncementsError {
    NoAnnouncementsFound,
    AnnouncementNotFound,
}

impl error::ResponseError for AnnouncementsError {
    fn status_code(&self) -> StatusCode {
        match self {
            AnnouncementsError::NoAnnouncementsFound | AnnouncementsError::AnnouncementNotFound => {
                StatusCode::NOT_FOUND
            }
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
