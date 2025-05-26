use crate::entities::{external_team_invitation, team_invites};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, delete};
use sea_orm::{EntityTrait, ModelTrait};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Team invite successfully deleted."),
        (status = 404, description = "Team invite not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "admin_delete_team_invite",
    tag = "admin/team_invites"
)]
#[delete("/delete/{invite_id}")]
pub async fn delete(
    app_state: Data<AppState>,
    invite_id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let active_team_invite = team_invites::Entity::find_by_id(invite_id.into_inner())
        .one(&app_state.database)
        .await?
        .ok_or(Error::TeamInviteNotFound)?;

    active_team_invite.delete(&app_state.database).await?;

    Ok(SuccessResponse::default().http_response())
}

#[utoipa::path(
    responses(
        (status = 200, description = "External team invite successfully deleted."),
        (status = 404, description = "External team invite not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/team_invites"
)]
#[delete("/delete_external/{invite_id}")]
pub async fn delete_external(
    app_state: Data<AppState>,
    invite_id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let active_external_invite =
        external_team_invitation::Entity::find_by_id(invite_id.into_inner())
            .one(&app_state.database)
            .await?
            .ok_or(Error::TeamInviteNotFound)?;

    active_external_invite.delete(&app_state.database).await?;

    Ok(SuccessResponse::default().http_response())
}
