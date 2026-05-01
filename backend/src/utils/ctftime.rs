use crate::entities::{flag_capture, teams};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::points_counter::SingleTeamStanding;
use actix_web::web::{Data, Query};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::{QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
pub struct CaptureLogEvent {
    pub id: i32,
    pub time: Option<i64>,
    pub r#type: Option<String>,
    pub team: String,
    pub victim: Option<String>,
    pub task: Option<String>,
    #[serde(rename = "pointsDelta")]
    pub points_delta: Option<usize>,
}

/// Team Standings for https://ctftime.org/json-scoreboard-feed
#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct TeamStandings {
    pub tasks: Vec<String>,
    pub standings: Vec<SingleTeamStanding>,
}

#[derive(serde::Deserialize, utoipa::ToSchema)]
#[allow(dead_code)]
pub struct CaptureLogQuery {
    #[serde(rename = "lastId")]
    pub last_id: Option<i32>,
}

pub async fn get_capture_log(
    app_state: Data<AppState>,
    query: Query<CaptureLogQuery>,
) -> Result<Vec<CaptureLogEvent>, Error> {
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

    Ok(captures
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
        .collect())
}
