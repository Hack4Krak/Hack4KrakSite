use actix_web::{get, web, HttpResponse};
use sea_orm::prelude::DateTime;
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::models::entities::{teams, users};
use crate::routes::teams::TeamError::NotFound;
use crate::utils::app_state;
use crate::utils::error::Error;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TeamWithMembers {
    pub team_name: String,
    pub leader_name: String,
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
    let team_model = teams::Entity::find()
        .filter(teams::Column::Name.contains(team_name.into_inner()))
        .find_also_related(users::Entity)
        .one(&app_state.database)
        .await?
        .ok_or(Error::Team(NotFound));

    let (team, users) = team_model?;

    let member_names: Vec<String> = users.into_iter().map(|user| user.username).collect();

    let team_response = TeamWithMembers {
        team_name: team.name,
        leader_name: team.leader_name,
        created_at: team.created_at,
        members: member_names,
    };
    Ok(HttpResponse::Ok().json(team_response))
}
