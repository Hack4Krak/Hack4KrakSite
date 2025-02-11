use crate::entities::{teams, users};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{delete, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RemoveUserModel {
    pub username: String,
}

#[utoipa::path(
    request_body = RemoveUserModel,
    responses(
        (status = 200, description = "User successfully removed from team."),
        (status = 403, description = "User is not a member of the team."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/management"
)]
#[delete("/kick_user")]
pub async fn kick_user(
    app_state: web::Data<app_state::AppState>,
    model: web::Json<RemoveUserModel>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let user_to_remove = users::Model::find_by_username(&app_state.database, &model.username)
        .await?
        .ok_or(Error::UserNotFound)?;

    teams::Model::kick_user(&app_state.database, user_to_remove, user).await?;

    Ok(SuccessResponse::default().http_response())
}
