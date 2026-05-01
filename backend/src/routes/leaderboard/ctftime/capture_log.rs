use crate::utils::app_state::AppState;
use crate::utils::ctftime::{CaptureLogEvent, CaptureLogQuery, get_capture_log};
use actix_web::web::Data;
use actix_web::{Error, HttpResponse, get};

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
#[get("/capture-log")]
pub async fn capture_log(
    app_state: Data<AppState>,
    query: actix_web::web::Query<CaptureLogQuery>,
) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(get_capture_log(app_state, query).await?))
}
