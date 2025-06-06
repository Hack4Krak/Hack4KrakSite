use crate::models::task::{TaskDisplay, TaskMeta};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct SimpleTask {
    #[serde(flatten)]
    pub description: TaskMeta,
    pub display: TaskDisplay,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Tasks successfully retrieved.", body = Vec<SimpleTask>),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "task_list",
    tag = "tasks"
)]
#[get("/list")]
pub async fn list(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let manager = &app_state.task_manager;

    let mut tasks = Vec::new();
    for task in manager.tasks.iter() {
        tasks.push(SimpleTask {
            description: task.meta.clone(),
            display: task.display.clone(),
        })
    }

    Ok(HttpResponse::Ok().json(tasks))
}
