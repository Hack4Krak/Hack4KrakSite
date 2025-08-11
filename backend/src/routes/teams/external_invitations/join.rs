use crate::AuthMiddleware;
use crate::entities::{external_team_invitation, users};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::Json;
use actix_web::{HttpResponse, post, web};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use std::ops::Deref;
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct JoinExternalModel {
    #[validate(length(equal = 6))]
    pub code: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully joined team."),
        (status = 403, description = "User already belongs to team."),
        (status = 404, description = "Invitation not found."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "join_external",
    security(
        ("access_token" = [])
    ),
    tag = "teams/external_invitations"
)]
#[post("/join", wrap = "AuthMiddleware::with_user()")]
pub async fn join(
    app_state: web::Data<app_state::AppState>,
    user: users::Model,
    Validated(team_code): Validated<Json<JoinExternalModel>>,
) -> Result<HttpResponse, Error> {
    let registration_config = app_state.task_manager.registration_config.read().await;
    external_team_invitation::Model::accept_invitation(
        &app_state.database,
        registration_config.deref(),
        team_code.into_inner().code,
        user,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
