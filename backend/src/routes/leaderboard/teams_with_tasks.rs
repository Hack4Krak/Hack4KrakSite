use crate::entities::teams;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::leaderboard_freeze::active_freeze_cutoff;
use crate::utils::points_counter::PointsCounter;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use chrono::Utc;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq)]
pub struct TeamRankingWithTasks {
    pub team_id: Uuid,
    pub team_name: String,
    pub current_points: usize,
    pub captured_flags: usize,
    pub color: String,
    pub tasks: HashMap<String, DateTime>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved leaderboard", body = Vec<TeamRankingWithTasks>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "leaderboard"
)]
#[get("/teams_with_tasks")]
pub async fn teams_with_tasks(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let rankings = PointsCounter::calculate(&app_state).await?.get_rankings();
    let freeze_cutoff = {
        let event_config = app_state.task_manager.event_config.read().await;
        active_freeze_cutoff(Utc::now(), event_config.end_date)
    };

    let tasks_by_team = teams::Model::get_tasks_by_team(
        &app_state.database,
        rankings.iter().map(|ranking| ranking.team_id).collect(),
        freeze_cutoff,
    )
    .await?;
    let merged = rankings
        .into_iter()
        .map(|ranking| {
            let tasks = tasks_by_team
                .get(&ranking.team_id)
                .cloned()
                .unwrap_or_default();
            TeamRankingWithTasks {
                team_id: ranking.team_id,
                team_name: ranking.team_name,
                current_points: ranking.current_points,
                captured_flags: ranking.captured_flags,
                color: ranking.color,
                tasks,
            }
        })
        .collect::<Vec<_>>();

    Ok(HttpResponse::Ok().json(merged))
}
