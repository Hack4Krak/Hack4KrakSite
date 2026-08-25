use crate::entities::{teams, users};
use crate::models::user::validate_no_edge_whitespace;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{HttpResponse, patch, web};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug)]
pub struct ChangeLeaderModel {
    #[validate(custom(function = "validate_no_edge_whitespace"))]
    pub new_leader_username: String,
}

#[utoipa::path(
    request_body = ChangeLeaderModel,
    responses(
        (status = 200, description = "Team name successfully changed."),
        (status = 403, description = "User is not the leader of any team."),
        (status = 404, description = "Team not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/management"
)]
#[patch("/change_leader")]
pub async fn change_leader(
    app_state: web::Data<app_state::AppState>,
    Validated(model): Validated<web::Json<ChangeLeaderModel>>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let new_leader =
        users::Model::find_by_username(&app_state.database, &model.new_leader_username)
            .await?
            .ok_or(Error::UserNotFound)?;

    teams::Model::change_leader(&app_state.database, new_leader, user).await?;

    Ok(SuccessResponse::default().http_response())
}
