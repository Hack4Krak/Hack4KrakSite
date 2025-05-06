use crate::entities::sea_orm_active_enums::TeamStatus;
use crate::entities::{teams, users};
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::{HttpResponse, get, web};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use sea_orm::prelude::DateTime;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct MyTeamWithMembers {
    pub team_name: String,
    pub created_at: DateTime,
    pub members: Vec<TeamMember>,
    pub status: TeamStatus,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TeamMember {
    pub name: String,
    pub is_leader: bool,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Team successfully retrieved.", body = MyTeamWithMembers),
        (status = 403, description = "User doesn't belong to any team."),
        (status = 404, description = "Team not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/membership"
)]
#[get("/my_team", wrap = "AuthMiddleware::with_team_as_member()")]
pub async fn my_team(
    app_state: web::Data<app_state::AppState>,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    let users = users::Entity::find()
        .filter(users::Column::Team.eq(team.id))
        .all(&app_state.database)
        .await?;

    let members = users
        .into_iter()
        .map(|user| TeamMember {
            name: user.username,
            is_leader: user.is_leader,
        })
        .collect();

    let team_response = MyTeamWithMembers {
        team_name: team.name,
        created_at: team.created_at,
        members,
        status: team.status,
    };

    Ok(HttpResponse::Ok().json(team_response))
}
