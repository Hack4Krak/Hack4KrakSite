use actix_web::http::StatusCode;
use actix_web::{HttpResponse, error};
use std::string;
use thiserror::Error;
use utoipa_actix_web::scope;

mod assets;
mod background;
mod count;
mod description;
mod icon;
mod list;
mod story;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(list::list);
    cfg.service(icon::icon);
    cfg.service(story::story);
    cfg.service(background::background);
    cfg.service(description::description);
    cfg.service(scope("/assets").configure(assets::config));
    cfg.service(count::count);
}

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task ID may only contain a-Z, A-Z, 0-9, - and _")]
    InvalidTaskId,
    #[error("Could not load task asset for task \"{id}\"")]
    CouldNotLoadTaskAsset { id: String },
    #[error("Task \"{id}\" does not exists")]
    MissingTask { id: String },
    #[error("Could not load task description for task \"{id}\"")]
    CouldNotLoadTaskDescription { id: String },
    #[error("Error while reading task description: {0}")]
    ErrorWhileReadingDescription(#[from] string::FromUtf8Error),
}

impl error::ResponseError for TaskError {
    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::InvalidTaskId => StatusCode::BAD_REQUEST,
            TaskError::MissingTask { .. } => StatusCode::NOT_FOUND,
            TaskError::CouldNotLoadTaskAsset { .. }
            | TaskError::CouldNotLoadTaskDescription { .. }
            | TaskError::ErrorWhileReadingDescription(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
