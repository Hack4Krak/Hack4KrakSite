use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Debug, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum EventStatusStage {
    Before,
    During,
    After,
}

#[derive(Debug, Serialize, ToSchema)]
pub struct EventStatusResponse {
    pub stage: EventStatusStage,
    pub is_live: bool,
    pub is_before_event: bool,
    pub is_after_event: bool,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Event status returned", body = EventStatusResponse),
        (status = 500, description = "Internal server error."),
    ),
    tag = "event"
)]
#[get("/status")]
pub async fn status(
    app_state: Data<crate::utils::app_state::AppState>,
) -> Result<HttpResponse, Error> {
    let event_config = app_state.task_manager.event_config.read().await;
    let is_before_event = event_config.is_before_event();
    let is_after_event = event_config.is_after_event();
    let is_live = event_config.is_during_event();
    let stage = if is_before_event {
        EventStatusStage::Before
    } else if is_after_event {
        EventStatusStage::After
    } else {
        EventStatusStage::During
    };

    Ok(HttpResponse::Ok().json(EventStatusResponse {
        stage,
        is_live,
        is_before_event,
        is_after_event,
    }))
}
