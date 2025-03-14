use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

mod chart;
mod teams;
pub mod updates;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(chart::chart);
    config.service(teams::teams);
    config.route("/updates", actix_web::web::get().to(updates::sse_handler));
}

#[derive(Debug, Error)]
pub enum ScoreboardError {}

impl ResponseError for ScoreboardError {
    fn status_code(&self) -> StatusCode {
        todo!("Status codes not implemented yet")
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
