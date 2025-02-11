use crate::entities::{team_invites, teams, users};
use crate::routes::teams::TeamError::TeamNotFound;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{post, web, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully joined team."),
        (status = 403, description = "User already belongs to team."),
        (status = 404, description = "Invitation not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/invitations"
)]
#[post("/accept_invitation/{team_name}")]
pub async fn accept_invitation(
    app_state: web::Data<app_state::AppState>,
    user: users::Model,
    team_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let team = teams::Model::find_by_name(&app_state.database, &team_name)
        .await?
        .ok_or(Error::Team(TeamNotFound))?;

    team_invites::Model::accept_invitation(&app_state.database, team, user).await?;

    Ok(SuccessResponse::default().http_response())
}
