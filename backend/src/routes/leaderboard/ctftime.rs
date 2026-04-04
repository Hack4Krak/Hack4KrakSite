use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::points_counter::{PointsCounter, TeamStandings};
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved information.", body = TeamStandings),
        (status = 500, description = "Internal server error"),
    ),
    tag = "leaderboard"
)]
#[get("/ctftime-team-standings")]
pub async fn team_standings(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let points_counter = PointsCounter::calculate(&app_state).await?;

    Ok(HttpResponse::Ok().json(points_counter.to_standings(&app_state).await?))
}
