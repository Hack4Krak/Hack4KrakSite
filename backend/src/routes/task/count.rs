use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned the number of tasks.", body = u16),
    ),
    tag = "tasks"
)]
#[get("/count")]
pub async fn count(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok().json(app_state.task_manager.tasks.len() as u16))
}
