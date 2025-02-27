use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, patch};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateTeamModel {
    pub team_name: Option<String>,
    pub leader: Option<Uuid>,
    pub status: Option<teams::TeamStatus>,
}

#[utoipa::path(
    request_body = UpdateTeamModel,
    responses(
        (status = 200, description = "Team successfully updated."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_teams_update",
    tag = "admin/teams"
)]
#[patch("/update/{id}")]
pub async fn update(
    app_state: Data<app_state::AppState>,
    id: Path<Uuid>,
    update_team_json: Json<UpdateTeamModel>,
) -> Result<HttpResponse, Error> {
    teams::Model::update(
        &app_state.database,
        id.into_inner(),
        update_team_json.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}
