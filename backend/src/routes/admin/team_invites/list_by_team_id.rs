use super::TeamInviteListResponse;
use crate::entities::external_team_invitation;
use crate::entities::team_invites;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Team invites successfully fetched.", body = TeamInviteListResponse),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "admin_list_team_invites_for_team",
    tag = "admin/team_invites"
)]
#[get("/list_by_team_id/{team_id}")]
pub async fn list_by_team_id(
    app_state: Data<AppState>,
    team_id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let team_id = team_id.into_inner();

    let team_invites = team_invites::Entity::find()
        .filter(team_invites::Column::Team.eq(team_id))
        .all(&app_state.database)
        .await?;

    let external_team_invites = external_team_invitation::Entity::find()
        .filter(external_team_invitation::Column::Team.eq(team_id))
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(TeamInviteListResponse {
        team_invites,
        external_team_invites,
    }))
}
