use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned task name", body = String),
        (status = 400, description = "Invalid task id."),
        (status = 404, description = "Task not found."),
    ),
    tag = "tasks"
)]
#[get("/name/{task_id}")]
pub async fn name(app_state: Data<AppState>, task_id: Path<String>) -> Result<HttpResponse, Error> {
    let content = app_state
        .task_manager
        .get_task(&task_id)?
        .description
        .name
        .clone();

    Ok(HttpResponse::Ok().body(content))
}
