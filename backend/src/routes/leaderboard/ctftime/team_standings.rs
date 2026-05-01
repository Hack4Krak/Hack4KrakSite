use crate::utils::ctftime::TeamStandings;
use crate::utils::points_counter::PointsCounter;
use actix_web::web::Data;
use actix_web::{Error, HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved information.", body = TeamStandings),
        (status = 500, description = "Internal server error"),
    ),
    tag = "leaderboard"
)]
#[get("/team-standings")]
pub async fn team_standings(
    app_state: Data<crate::utils::app_state::AppState>,
) -> Result<HttpResponse, Error> {
    let points_counter = PointsCounter::calculate(&app_state).await?;

    Ok(HttpResponse::Ok().json(points_counter.to_ctftime_standings(&app_state).await?))
}
