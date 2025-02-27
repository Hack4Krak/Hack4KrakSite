use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, delete};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Confirmation code successfully cleared."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/teams"
)]
#[delete("/clear_confirmation_code/{id}")]
pub async fn clear_confirmation_code(
    app_state: Data<app_state::AppState>,
    id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    teams::Model::clear_confirmation_code(&app_state.database, id.into_inner()).await?;

    Ok(SuccessResponse::default().http_response())
}
