use crate::EventMiddleware;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned task solution", body = String),
        (status = 400, description = "Invalid task id."),
        (status = 404, description = "Task not found."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "tasks"
)]
#[get(
    "/solution/{task_id}",
    wrap = "EventMiddleware::allow_only_after_event()"
)]
pub async fn solution(
    app_state: Data<AppState>,
    request: HttpRequest,
    task_id: Path<String>,
) -> Result<HttpResponse, Error> {
    let content = app_state
        .task_manager
        .load_asset(&task_id, "solution.md")
        .await?;

    Ok(content.into_response(&request))
}
