use crate::entities::announcement;
use crate::models::announcement::{AnnouncementAction, AnnouncementResponse};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, post, web};

#[utoipa::path(
    request_body = AnnouncementAction,
    responses(
        (status = 201, description = "Announcement created successfully.", body = AnnouncementResponse),
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

    let model = if let AnnouncementAction::TaskStatusUpdate { .. } = action {
        app_state
            .task_manager
            .update_task_status(&app_state.database, &action)
            .await?
    } else {
        announcement::Model::create(&app_state.database, &action).await?
    };

    Ok(HttpResponse::Created().json(model.response()?))
}
