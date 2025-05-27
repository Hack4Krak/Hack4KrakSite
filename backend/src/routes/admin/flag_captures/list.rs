use super::FlagCaptureListResponse;
use crate::entities::flag_capture;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "Flag captures successfully fetched.", body = FlagCaptureListResponse),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_list_flag_captures",
    security(
        ("access_token" = [])
    ),
    tag = "admin/flag_captures"
)]
#[get("/list")]
pub async fn list(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let flag_captures = flag_capture::Entity::find()
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(FlagCaptureListResponse { flag_captures }))
}
