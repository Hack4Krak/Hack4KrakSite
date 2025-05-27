use crate::entities::external_team_invitation;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, delete};
use sea_orm::{EntityTrait, ModelTrait};
use uuid::Uuid;

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
