use actix_web::middleware::from_fn;
use actix_web::{get, web, HttpResponse};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;

use crate::models::entities::{teams, users};
use crate::routes::teams::team::TeamWithMembers;
use crate::routes::teams::TeamError::{TeamNotFound, UserDoesntBelongToAnyTeam};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::Claims;

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
    tag = "teams"
)]
#[get(
    "/my_team",
    wrap = "from_fn(crate::middlewares::auth_middleware::check_auth_middleware)"
)]
pub async fn my_team(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    let user = users::Entity::find_by_id(claim_data.id)
        .one(&app_state.database)
        .await?
        .ok_or(Error::Unauthorized)?;

    let Some(team_id) = user.team else {
        return Err(Error::Team(UserDoesntBelongToAnyTeam {
            username: user.username,
        }))?;
    };

    let team = teams::Entity::find_by_id(team_id)
        .one(&app_state.database)
        .await?
        .ok_or(Error::Team(TeamNotFound))?;

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
