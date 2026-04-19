use crate::services::identification::{IdentificationService, IdentifiedUserInfo};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get, web};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "User successfully identified.", body = IdentifiedUserInfo),
        (status = 404, description = "No user found with the given identification UUID."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_identification_identify",
    tag = "admin/identification"
)]
#[get("/identify/{id}")]
pub async fn identify(
    app_state: Data<AppState>,
    id: web::Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_info =
        IdentificationService::identify_user(&app_state.database, id.into_inner()).await?;

    Ok(HttpResponse::Ok().json(user_info))
}
