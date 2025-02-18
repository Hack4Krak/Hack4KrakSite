use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{patch, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct ChangeNameModel {
    #[validate(length(min = 3, max = 32))]
    pub new_name: String,
}

#[utoipa::path(
    request_body = ChangeNameModel,
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
#[patch("/rename")]
pub async fn rename(
    app_state: web::Data<app_state::AppState>,
    change_name_model: actix_web_validator::Json<ChangeNameModel>,
    team: teams::Model,
) -> Result<HttpResponse, Error> {
    teams::Model::rename(
        &app_state.database,
        change_name_model.new_name.clone(),
        team,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
