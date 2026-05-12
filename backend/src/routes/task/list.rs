use crate::services::task_manager::TaskWithStatus;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Tasks successfully retrieved.", body = Vec<TaskWithStatus>),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "task_list",
    tag = "tasks"
)]
#[get("/list")]
pub async fn list(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let manager = &app_state.task_manager;

    let tasks = manager.available_tasks().await;

    Ok(HttpResponse::Ok().json(tasks))
}
