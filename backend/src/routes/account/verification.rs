use crate::entities::users;
use crate::services::verification::{UserVerificationInfo, VerificationService};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Verification information received.", body = UserVerificationInfo),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "account_verification",
    tag = "account"
)]
#[get("/verification")]
pub async fn verification(
    app_state: Data<AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let verification_info = VerificationService::user_verification_info(
        &app_state.database,
        &app_state.task_manager,
        &user,
    )
    .await?;

    Ok(HttpResponse::Ok().json(verification_info))
}
