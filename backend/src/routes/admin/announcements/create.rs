use crate::entities::announcement;
use crate::models::announcement::AnnouncementAction;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::Data;
use actix_web::{HttpResponse, post, web};

#[utoipa::path(
    request_body = AnnouncementAction,
    responses(
        (status = 200, description = "Announcement created successfully.", body = announcement::Model),
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "admin-announcements-create",
    tag = "admin/announcements"
)]
#[post("/")]
pub async fn create(
    app_state: Data<AppState>,
    action: web::Json<AnnouncementAction>,
) -> Result<HttpResponse, Error> {
    let action = action.into_inner();

    if let AnnouncementAction::TaskStatusUpdate { .. } = action {
        app_state
            .task_manager
            .update_task_status(&app_state.database, &action)
            .await?;
    } else {
        announcement::Model::create(&app_state.database, &action).await?;
    }
    Ok(SuccessResponse::default().http_response())
}
