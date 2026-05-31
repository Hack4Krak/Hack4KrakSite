use crate::models::task_manager::event_config::{EventConfig, EventStageType};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use chrono::{DateTime, FixedOffset};
use serde::Serialize;
use utoipa::ToSchema;

#[derive(Serialize, ToSchema)]
struct EventInfoResponse {
    #[serde(flatten)]
    event: EventConfig,
    start_date: Option<DateTime<FixedOffset>>,
    end_date: Option<DateTime<FixedOffset>>,
    is_started: bool,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned event config", body = EventInfoResponse),
    ),
    tag = "event"
)]
#[get("/info")]
pub async fn info(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let event_config = app_state.task_manager.event_config.read().await;

    Ok(HttpResponse::Ok().json(EventInfoResponse {
        start_date: event_config.event_stage_start(EventStageType::EventStart),
        end_date: event_config.event_stage_start(EventStageType::EventEnd),
        event: event_config.clone(),
        is_started: !event_config.is_before_event(),
    }))
}
