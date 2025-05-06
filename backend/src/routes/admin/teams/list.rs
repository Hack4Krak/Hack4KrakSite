use crate::entities::teams;
use crate::models::teams::TeamWithMembers;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Teams successfully fetched.", body = Vec<TeamWithMembers>),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_teams_list",
    tag = "admin/teams"
)]
#[get("/list")]
pub async fn list(app_state: Data<app_state::AppState>) -> Result<HttpResponse, Error> {
    let teams = teams::Model::list(&app_state.database).await?;

    Ok(HttpResponse::Ok().json(teams))
}
