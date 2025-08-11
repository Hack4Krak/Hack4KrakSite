use crate::middlewares::event::EventMiddleware;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Event is in progress"),
        (status = 403, description = "You cannot access the event"),
    ),
    tag = "event"
)]
#[get("/status", wrap = "EventMiddleware::disallow_before_event()")]
pub async fn status() -> Result<HttpResponse, Error> {
    Ok(SuccessResponse::default().http_response())
}
