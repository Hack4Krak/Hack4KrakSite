use crate::entities::users;
use crate::services::authorization::UserIdentificationInfo;
use crate::services::identification::IdentificationService;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Identification information received.", body = UserIdentificationInfo),
        (status = 401, description = "Unauthorized."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "account_identification",
    tag = "account"
)]
#[get("/identification")]
pub async fn identification(
    app_state: Data<AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let identification_info = IdentificationService::user_identification_info(
        &app_state.database,
        &app_state.task_manager,
        &user,
    )
    .await?;

    Ok(HttpResponse::Ok().json(identification_info))
}
