use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned icon image.", body = String),
        (status = 400, description = "Invalid task id."),
        (status = 404, description = "Task not found."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "tasks"
)]
#[get("/icon/{task_id}")]
pub async fn icon(app_state: Data<AppState>, task_id: Path<String>) -> Result<HttpResponse, Error> {
    let manager = app_state
        .task_manager
        .read()
        .map_err(|_| Error::PoisonedLock)?;
    let content = manager.load_asset(&task_id, "pictures/icon.png").await?;

    Ok(HttpResponse::Ok().body(content))
}
