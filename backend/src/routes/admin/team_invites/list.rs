use crate::entities::{external_team_invitation, team_invites};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
#[utoipa::path(
    responses(
        (status = 200, description = "Team invites successfully fetched.", body = Vec<team_invites::Model>),
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

    Ok(HttpResponse::Ok().json(team_invites))
}

use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "External team invites successfully fetched.", body = Vec<external_team_invitation::Model>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/team_invites"
)]
#[get("/list_external")]
pub async fn list_external(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let external_team_invites = external_team_invitation::Entity::find()
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(external_team_invites))
}
