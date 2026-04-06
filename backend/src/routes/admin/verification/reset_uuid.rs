use crate::services::verification::VerificationService;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, post};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "Verification UUID successfully reset and email sent."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_verification_reset_uuid",
    tag = "admin/verification"
)]
#[post("/reset/{user_id}")]
pub async fn reset_uuid(
    app_state: Data<AppState>,
    user_id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    VerificationService::reset_verification_id(&app_state, *user_id).await?;

    Ok(SuccessResponse::default().http_response())
}
