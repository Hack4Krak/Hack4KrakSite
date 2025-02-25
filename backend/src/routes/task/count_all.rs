use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned the number of tasks.", body = u16),
        (status = 500, description = "Internal server error.")
    ),
    tag = "tasks"
)]
#[get("/count_all")]
pub async fn count_all(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(app_state.task_manager.tasks.len() as u16))
}
