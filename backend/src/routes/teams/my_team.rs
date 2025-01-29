use actix_web::middleware::from_fn;
use actix_web::{get, web, HttpResponse};
use sea_orm::EntityTrait;

use crate::models::entities::{teams, users};
use crate::routes::teams::view_team::TeamWithMembers;
use crate::routes::teams::TeamError::UserDoesntBelongToAnyTeam;
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

    if user.team_name.is_none() {
        return Err(Error::Team(UserDoesntBelongToAnyTeam))?;
    }

    let team_response =
        teams::Model::get_team(&app_state.database, user.team_name.unwrap()).await?;

    Ok(HttpResponse::Ok().json(team_response))
}
