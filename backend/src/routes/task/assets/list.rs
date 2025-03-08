use crate::models::task::TaskAsset;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::{HttpResponse, get, web};

#[utoipa::path(
    responses(
        (status = 200, description = "List of task assets.", body = Vec<TaskAsset>),
        (status = 404, description = "Task does not exist."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "task_assets_list",
    tag = "task/assets"
)]
#[get("/list/{task_id}")]
pub async fn list(
    app_state: web::Data<app_state::AppState>,
    task_id: web::Path<String>,
) -> Result<HttpResponse, Error> {
    let task = app_state.task_manager.get_task(&task_id.into_inner())?;

    Ok(HttpResponse::Ok().json(&task.assets))
}
