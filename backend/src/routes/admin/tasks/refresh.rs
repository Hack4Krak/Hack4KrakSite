use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{post, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "Tasks successfully refreshed."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/tasks"
)]
#[post("/refresh")]
pub async fn refresh(app_state: Data<app_state::AppState>) -> Result<HttpResponse, Error> {
    app_state.task_manager.refresh().await;

    Ok(HttpResponse::Ok().finish())
}
