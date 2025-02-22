use crate::entities::{team_invites, teams};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Path};
use actix_web::{delete, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Invitation successfully revoked."),
        (status = 404, description = "Invitation not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/management"
)]
#[delete("/teams/management/revoke_invitation/{username}")]
pub async fn revoke_invitation(
    app_state: Data<app_state::AppState>,
    username: Path<String>,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    team_invites::Model::revoke_invitation(&app_state.database, &username, team.id).await?;

    Ok(SuccessResponse::default().http_response())
}
