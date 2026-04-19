use crate::entities::{flag_capture, teams};
use crate::utils::error::Error;
use crate::utils::points_counter::{CaptureLogEvent, PointsCounter, TeamStandings};
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::{QueryFilter, QueryOrder};
use std::collections::HashMap;
use std::sync::Arc;
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved information.", body = TeamStandings),
        (status = 500, description = "Internal server error"),
    ),
    tag = "leaderboard"
)]
#[get("/ctftime-team-standings")]
pub async fn team_standings(
    app_state: Data<crate::utils::app_state::AppState>,
) -> Result<HttpResponse, Error> {
    let points_counter = PointsCounter::calculate(&app_state).await?;

    Ok(HttpResponse::Ok().json(points_counter.to_standings(&app_state).await?))
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
#[allow(dead_code)]
pub struct CaptureLogQuery {
    #[serde(rename = "lastId")]
    pub last_id: Option<i32>,
}

#[utoipa::path(
    params(
        ("lastId" = Option<i32>, Query, description = "ID of the last event received")
    ),
    responses(
        (status = 200, description = "Successfully retrieved capture log.", body = [CaptureLogEvent]),
        (status = 500, description = "Internal server error"),
    ),
    tag = "leaderboard"
)]
#[get("/ctftime-capture-log")]
pub async fn capture_log(
    app_state: Data<crate::utils::app_state::AppState>,
    query: actix_web::web::Query<CaptureLogQuery>,
) -> Result<HttpResponse, Error> {
    let last_id = query.last_id.unwrap_or(0);
    let app_state = Arc::new(app_state.into_inner());

    let captures: Vec<flag_capture::Model> = flag_capture::Entity::find()
        .filter(flag_capture::Column::Id.gt(last_id))
        .order_by_asc(flag_capture::Column::Id)
        .all(&app_state.database)
        .await?;

    let team_names: HashMap<Uuid, String> = teams::Entity::find()
        .all(&app_state.database)
        .await?
        .into_iter()
        .map(|t| (t.id, t.name))
        .collect();

    let events: Vec<CaptureLogEvent> = captures
        .into_iter()
        .map(|capture| CaptureLogEvent {
            id: capture.id,
            time: Some(capture.submitted_at.and_utc().timestamp()),
            r#type: Some("taskCorrect".to_string()),
            team: team_names.get(&capture.team).cloned().unwrap_or_default(),
            task: Some(capture.task),
            points_delta: None,
            ..Default::default()
        })
        .collect();

    Ok(HttpResponse::Ok().json(events))
}
