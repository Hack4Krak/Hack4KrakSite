use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use thiserror::Error;

mod background;
mod icon;
mod list;
mod story;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(list::list);
    cfg.service(icon::icon);
    cfg.service(story::story);
    cfg.service(background::background);
}

#[derive(Debug, Error)]
pub enum TaskError {
    #[error("Task ID may only contain a-Z, A-Z, 0-9, - and _")]
    InvalidTaskId,
    #[error("Could not load task icon for task \"{id}\"")]
    CouldNotLoadTaskIcon { id: String },
    #[error("Task \"{id}\" does not exists")]
    MissingTask { id: String },
}

impl error::ResponseError for TaskError {
    fn status_code(&self) -> StatusCode {
        match self {
            TaskError::InvalidTaskId => StatusCode::BAD_REQUEST,
            TaskError::MissingTask { .. } => StatusCode::NOT_FOUND,
            TaskError::CouldNotLoadTaskIcon { .. } => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
