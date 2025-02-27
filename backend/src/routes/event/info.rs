use crate::models::task::EventConfig;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use std::ops::Deref;

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned event config", body = EventConfig),
    ),
    tag = "event"
)]
#[get("/info")]
pub async fn info(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let event_config = app_state.task_manager.event_config.lock().await;
    Ok(HttpResponse::Ok().json(event_config.deref()))
}
