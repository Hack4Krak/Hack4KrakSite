use crate::services::verification::{VerificationService, VerifiedUserInfo};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use serde::Deserialize;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Deserialize, ToSchema)]
pub struct IdentifyRequest {
    pub verification_id: Uuid,
}

#[utoipa::path(
    request_body = IdentifyRequest,
    responses(
        (status = 200, description = "User successfully identified.", body = VerifiedUserInfo),
        (status = 404, description = "No user found with the given verification UUID."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_verification_identify",
    tag = "admin/verification"
)]
#[post("/identify")]
pub async fn identify(
    app_state: Data<AppState>,
    body: Json<IdentifyRequest>,
) -> Result<HttpResponse, Error> {
    let user_info =
        VerificationService::identify_user(&app_state.database, body.verification_id).await?;

    Ok(HttpResponse::Ok().json(user_info))
}
