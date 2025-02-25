use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully retrieved completed tasks.", body = Vec<String>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/membership",
)]
#[get("/completed_tasks")]
pub async fn completed_tasks(
    app_state: Data<app_state::AppState>,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    let tasks = teams::Model::get_completed_tasks(&app_state.database, team.id).await?;

    Ok(HttpResponse::Ok().json(tasks))
}
