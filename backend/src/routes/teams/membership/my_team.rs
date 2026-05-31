use crate::entities::sea_orm_active_enums::TeamStatus;
use crate::entities::{team_invites, users};
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
    pub invited_users: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TeamMember {
    pub name: String,
    pub is_leader: bool,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Team successfully retrieved.", body = Option<MyTeamWithMembers>),
        (status = 404, description = "Team not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/membership"
)]
#[get("/my_team", wrap = "AuthMiddleware::with_user()")]
pub async fn my_team(
    app_state: web::Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let Some(team) = user.get_team(&app_state.database).await? else {
        return Ok(HttpResponse::Ok().json(None::<MyTeamWithMembers>));
    };

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

    let invited_users = if user.is_leader {
        Some(team_invites::Model::get_invited_users(&app_state.database, team.id).await?)
    } else {
        None
    };

    let team_response = MyTeamWithMembers {
        team_name: team.name,
        created_at: team.created_at,
        members,
        status: team.status,
        invited_users,
    };

    Ok(HttpResponse::Ok().json(team_response))
}
