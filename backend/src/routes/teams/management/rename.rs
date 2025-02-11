use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{patch, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ChangeNameModel {
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
    change_name_model: web::Json<ChangeNameModel>,
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
