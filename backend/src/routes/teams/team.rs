use crate::models::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::{get, web, HttpResponse};
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TeamWithMembers {
    pub team_name: String,
    pub created_at: DateTime,
    pub members: Vec<String>,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Team successfully retrieved.", body = TeamWithMembers),
        (status = 404, description = "Team not found."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "teams"
)]
#[get("/team/{team_name}")]
pub async fn view_team(
    app_state: web::Data<app_state::AppState>,
    team_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let team_response = teams::Model::get_team(&app_state.database, team_name.into_inner()).await?;
    Ok(HttpResponse::Ok().json(team_response))
}
