use crate::entities::teams;
use crate::entities::teams::UpdatableModel;
use crate::routes::teams::TeamError::TeamNotFound;
use crate::utils::app_state;
use crate::utils::colors::is_valid_hex;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, patch};
use sea_orm::{ActiveModelTrait, EntityTrait};
use uuid::Uuid;

#[utoipa::path(
    request_body = UpdatableModel,
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
    update_team_json: Json<UpdatableModel>,
) -> Result<HttpResponse, Error> {
    let team = teams::Entity::find_by_id(*id)
        .one(&app_state.database)
        .await?
        .ok_or(Error::Team(TeamNotFound))?;

    let updatable_team_model = update_team_json.into_inner();

    if let Some(color) = &updatable_team_model.color
        && !is_valid_hex(&color.to_uppercase())
    {
        return Err(Error::InvalidColorFormat);
    }

    updatable_team_model
        .update(team)
        .save(&app_state.database)
        .await?;

    Ok(SuccessResponse::default().http_response())
}
