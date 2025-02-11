use crate::routes::task::TaskError;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned task description", body = String),
        (status = 400, description = "Invalid task id."),
        (status = 404, description = "Task not found."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "tasks"
)]
#[get("/description/{task_id}")]
pub async fn description(
    app_state: Data<AppState>,
    task_id: Path<String>,
) -> Result<HttpResponse, Error> {
    let content_bytes = app_state
        .task_manager
        .load_asset(&task_id, "description.md")
        .await?;
    let content =
        String::from_utf8(content_bytes).map_err(TaskError::ErrorWhileReadingDescription)?;

    Ok(HttpResponse::Ok().body(content))
}
