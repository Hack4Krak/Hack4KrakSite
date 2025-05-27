use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::QueryFilter;
// I need to import it directly, so I can name it differently to make OpenAPI generates correctly.
use crate::entities::flag_capture;
use crate::entities::flag_capture::Model as FlagCaptureModel;
use crate::routes::admin::flag_captures::FlagCaptureListResponse;

#[utoipa::path(
    responses(
        (status = 200, description = "Flag captures for task successfully fetched.", body = Vec<FlagCaptureModel>),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/flag_captures"
)]
#[get("/list_by_task/{task_id}")]
pub async fn list_by_task(
    app_state: Data<AppState>,
    task_id: Path<String>,
) -> Result<HttpResponse, Error> {
    let flag_captures = flag_capture::Entity::find()
        .filter(flag_capture::Column::Task.eq(task_id.into_inner()))
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(FlagCaptureListResponse { flag_captures }))
}
