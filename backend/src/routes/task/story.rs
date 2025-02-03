use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::task::TaskStory;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned dialogues for this task.", body = Vec<TaskStory>),
        (status = 400, description = "Invalid task id."),
        (status = 404, description = "Task not found."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "tasks"
)]
#[get("/story/{task_id}")]
pub async fn story(
    app_state: Data<AppState>,
    task_id: Path<String>,
) -> Result<HttpResponse, Error> {
    let manager = app_state
        .task_manager
        .read()
        .map_err(|_| Error::PoisonedLock)?;
    let task = manager.get_task(&task_id)?;

    Ok(HttpResponse::Ok().json(&task.story))
}
