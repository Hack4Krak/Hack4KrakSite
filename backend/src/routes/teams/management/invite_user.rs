use crate::entities::{team_invites, teams, users};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddUserModel {
    pub username: String,
}

#[utoipa::path(
    request_body = AddUserModel,
    responses(
        (status = 200, description = "User successfully invited to team."),
        (status = 403, description = "User already belongs to team."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = ["leader"])
    ),
    tag = "teams/management"
)]
#[post("/invite_user")]
pub async fn invite_user(
    app_state: web::Data<app_state::AppState>,
    model: web::Json<AddUserModel>,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    let invited_user = users::Model::find_by_username(&app_state.database, &model.username)
        .await?
        .ok_or(Error::UserNotFound)?;

    let event_config = app_state.task_manager.event_config.lock().await;
    team_invites::Model::invite_user(
        &app_state.database,
        event_config.deref(),
        invited_user,
        team,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
