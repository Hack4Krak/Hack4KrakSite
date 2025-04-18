use actix_web::http::StatusCode;
use actix_web::{HttpResponse, error};
use hack4krak_macros::error_with_messages;
use std::string;
use utoipa_actix_web::scope;

mod assets;
mod background;
mod count;
mod description;
mod icon;
mod list;
mod solution;
mod story;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(list::list);
    config.service(icon::icon);
    config.service(story::story);
    config.service(background::background);
    config.service(description::description);
    config.service(solution::solution);
    config.service(scope("/assets").configure(assets::config));
    config.service(count::count);
}

#[error_with_messages]
pub enum TaskError {
    InvalidTaskId,
    CouldNotLoadTaskAsset { id: String },
    MissingTask { id: String },
    ErrorWhileReadingDescription(#[from] string::FromUtf8Error),
}

impl error::ResponseError for TaskError {
    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::InvalidTaskId => StatusCode::BAD_REQUEST,
            TaskError::MissingTask { .. } => StatusCode::NOT_FOUND,
            TaskError::CouldNotLoadTaskAsset { .. }
            | TaskError::ErrorWhileReadingDescription(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
