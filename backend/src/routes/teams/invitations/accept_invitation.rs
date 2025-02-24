use crate::entities::{team_invites, teams, users};
use crate::routes::teams::TeamError::TeamNotFound;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{HttpResponse, post, web};
use std::ops::Deref;

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

    let event_config = app_state.task_manager.event_config.lock().await;
    team_invites::Model::accept_invitation(&app_state.database, event_config.deref(), team, user)
        .await?;

    Ok(SuccessResponse::default().http_response())
}
