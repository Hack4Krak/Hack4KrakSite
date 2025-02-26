use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

mod chart;
mod teams;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(chart::chart);
    cfg.service(teams::teams);
}

#[derive(Debug, Error)]
pub enum ScoreboardError {}

impl ResponseError for ScoreboardError {
    fn status_code(&self) -> StatusCode {
        todo!()
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
