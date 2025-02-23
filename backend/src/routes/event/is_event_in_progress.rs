use crate::middlewares::event::EventMiddleware;
use crate::utils::error::Error;
use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Event is in progress"),
        (status = 500, description = "Internal server error"),
    ),
    tag = "event"
)]
#[get("/is_event_in_progress", wrap = "EventMiddleware::allow_after_event()")]
pub async fn is_event_in_progress() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().body("Event is in progress"))
}
