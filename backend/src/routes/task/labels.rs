use crate::models::task_manager::task_config::Label;
use crate::utils::app_state::AppState;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully returned task labels list.", body = Vec<Label>),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "task_labels",
    tag = "tasks"
)]
#[get("/labels")]
pub async fn labels(app_state: Data<AppState>) -> HttpResponse {
    let labels = app_state
        .task_manager
        .labels_config
        .read()
        .await
        .labels
        .clone();
    HttpResponse::Ok().json(labels)
}
