use crate::models::task::ParticipantTag;
use crate::utils::app_state::AppState;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully returned participant tags list.", body = Vec<ParticipantTag>),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "event_participant_tags",
    tag = "event"
)]
#[get("/participant-tags")]
pub async fn participant_tags(app_state: Data<AppState>) -> HttpResponse {
    let tags = app_state
        .task_manager
        .participant_tags_config
        .read()
        .await
        .participant_tags
        .clone();
    HttpResponse::Ok().json(tags)
}
