use crate::services::verification::VerificationService;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct ApplyTagRequest {
    pub verification_id: Uuid,
    pub tag_id: String,
}

#[utoipa::path(
    request_body = ApplyTagRequest,
    responses(
        (status = 200, description = "Tag successfully applied."),
        (status = 404, description = "User or tag not found."),
        (status = 409, description = "User already has this tag applied."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_verification_apply_tag",
    tag = "admin/verification"
)]
#[post("/apply-tag")]
pub async fn apply_tag(
    app_state: Data<AppState>,
    body: Json<ApplyTagRequest>,
) -> Result<HttpResponse, Error> {
    VerificationService::apply_tag(
        &app_state.database,
        &app_state.task_manager,
        body.verification_id,
        &body.tag_id,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
