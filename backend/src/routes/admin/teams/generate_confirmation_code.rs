use crate::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, patch};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Confirmation code successfully generated."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/teams"
)]
#[patch("/generate_confirmation_code/{id}")]
pub async fn generate_confirmation_code(
    app_state: Data<app_state::AppState>,
    id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    teams::Model::generate_confirmation_code(&app_state.database, id.into_inner()).await?;

    Ok(SuccessResponse::default().http_response())
}
