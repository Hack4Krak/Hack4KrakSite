use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::points_counter::{Chart, PointsCounter};
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly submitted flag.", body = Chart),
        (status = 500, description = "Internal server error"),
    ),
    tag = "leaderboard"
)]
#[get("/chart")]
pub async fn chart(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let points_counter = PointsCounter::work(&app_state.database).await?;

    Ok(HttpResponse::Ok().json(points_counter.to_chart()))
}
