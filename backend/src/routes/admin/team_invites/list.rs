use crate::entities::{external_team_invitation, team_invites};
use crate::routes::admin::team_invites::TeamInviteListResponse;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "Team invites successfully fetched.", body = TeamInviteListResponse),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_list_team_invites",
    security(
        ("access_token" = [])
    ),
    tag = "admin/team_invites"
)]
#[get("/list")]
pub async fn list(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let team_invites = team_invites::Entity::find()
        .all(&app_state.database)
        .await?;

    let external_team_invites = external_team_invitation::Entity::find()
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(TeamInviteListResponse {
        team_invites,
        external_team_invites,
    }))
}
