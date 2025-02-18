use crate::entities::{teams, users};
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::{get, web, HttpResponse};
use sea_orm::prelude::DateTime;
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
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

    let members = users.into_iter().map(|user| user.username).collect();

    let team_response = TeamWithMembers {
        team_name: team.name,
        created_at: team.created_at,
        members,
    };

    Ok(HttpResponse::Ok().json(team_response))
}
