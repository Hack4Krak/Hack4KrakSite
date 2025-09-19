use crate::entities::teams;
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::points_counter::PointsCounter;
use actix_web::{HttpResponse, get, web};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TeamStats {
    pub rank: Option<(usize, usize)>,
    pub captured_flags: usize,
    pub remaining_flags: usize,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Team successfully retrieved.", body = TeamStats),
        (status = 403, description = "User doesn't belong to any team."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/membership"
)]
#[get("/stats", wrap = "AuthMiddleware::with_team_as_member()")]
pub async fn stats(
    app_state: web::Data<app_state::AppState>,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    let completed_tasks = teams::Model::get_completed_tasks(&app_state.database, team.id)
        .await?
        .len();
    let all_tasks = app_state.task_manager.tasks.len();
    let points_counter = PointsCounter::work(&app_state).await?;

    Ok(HttpResponse::Ok().json(TeamStats {
        rank: points_counter.team_rank(&team.name),
        captured_flags: completed_tasks,
        remaining_flags: all_tasks - completed_tasks,
    }))
}
