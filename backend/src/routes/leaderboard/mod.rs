use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use hack4krak_macros::error_with_messages;

mod chart;
mod teams;
mod updates;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(chart::chart);
    config.service(teams::teams);
    config.route("/updates", actix_web::web::get().to(updates::sse_handler));
}

#[error_with_messages]
pub enum ScoreboardError {}

impl ResponseError for ScoreboardError {
    fn status_code(&self) -> StatusCode {
        unreachable!()
    }

    fn error_response(&self) -> HttpResponse {
        crate::utils::error::error_response_builder(self)
    }
}
