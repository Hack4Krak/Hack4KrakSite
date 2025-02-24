use crate::models::task::TaskStory;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};

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
    let task = app_state.task_manager.get_task(&task_id)?;

    Ok(HttpResponse::Ok().json(&task.story))
}
