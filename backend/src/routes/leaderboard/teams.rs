use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::points_counter::{PointsCounter, TeamFinalPoints};
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly submitted flag.", body = Vec<TeamFinalPoints>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "leaderboard"
)]
#[get("/teams")]
pub async fn teams(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let points_counter = PointsCounter::work(&app_state.database).await?;

    Ok(HttpResponse::Ok().json(points_counter.get_final_team_points()))
}
