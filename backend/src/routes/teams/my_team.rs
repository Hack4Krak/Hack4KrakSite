use actix_web::middleware::from_fn;
use actix_web::{get, web, HttpResponse};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait};

use crate::models::entities::{teams, users};
use crate::routes::teams::view_team::TeamWithMembers;
use crate::routes::teams::TeamError::{NotFound, UserDoesntBelongToAnyTeam};
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
    let user = users::Entity::find_by_id(claim_data.email)
        .one(&app_state.database)
        .await?
        .ok_or(Error::Unauthorized)?;

    if user.team_name.is_none() {
        return Err(Error::Team(UserDoesntBelongToAnyTeam))?;
    }

    let team_model = teams::Entity::find()
        .filter(teams::Column::Name.eq(user.team_name))
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
