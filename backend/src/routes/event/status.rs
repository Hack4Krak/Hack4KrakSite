use crate::middlewares::event::EventMiddleware;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Event is in progress"),
    ),
    tag = "event"
)]
#[get("/status", wrap = "EventMiddleware::allow_after_event()")]
pub async fn status() -> Result<HttpResponse, Error> {
    Ok(SuccessResponse::default().http_response())
}
