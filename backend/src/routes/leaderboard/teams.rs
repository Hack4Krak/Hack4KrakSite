use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::points_counter::{PointsCounter, TeamRanking};
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved leaderboard", body = Vec<TeamRanking>),
        (status = 500, description = "Internal server error"),
    ),
    tag = "leaderboard"
)]
#[get("/teams")]
pub async fn teams(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let points_counter = PointsCounter::calculate(&app_state).await?;

    Ok(HttpResponse::Ok().json(points_counter.get_rankings()))
}
