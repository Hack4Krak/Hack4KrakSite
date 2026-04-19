use crate::services::identification::IdentificationService;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, post};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Identification code successfully reset and email sent."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_identification_reset_identification_code",
    tag = "admin/identification"
)]
#[post("/reset/{user_id}")]
pub async fn reset_identification_code(
    app_state: Data<AppState>,
    user_id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    IdentificationService::reset_identification_code(&app_state, *user_id).await?;

    Ok(SuccessResponse::default().http_response())
}
