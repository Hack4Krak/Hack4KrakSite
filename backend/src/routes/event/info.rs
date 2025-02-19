use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{get, HttpResponse};
use std::ops::Deref;

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned background image", body = String),
    ),
    tag = "event"
)]
#[get("/info")]
pub async fn info(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let event_config = app_state.task_manager.event_config.lock().await;
    Ok(HttpResponse::Ok().json(event_config.deref()))
}
