use crate::models::task::RegistrationConfig;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use std::ops::Deref;

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned registration config", body = RegistrationConfig),
    ),
    tag = "event"
)]
#[get("/registration")]
pub async fn registration(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let registration_config = app_state.task_manager.registration_config.lock().await;
    Ok(HttpResponse::Ok().json(registration_config.deref()))
}
