use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpRequest, HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly returned icon image.", body = String),
        (status = 400, description = "Invalid label id."),
        (status = 404, description = "Label not found."),
        (status = 500, description = "Internal server error.")
    ),
)]
#[get("/label/{label_id}")]
pub async fn label(
    app_state: Data<AppState>,
    label_id: Path<String>,
    request: HttpRequest,
) -> Result<HttpResponse, Error> {
    let content = app_state
        .task_manager
        .labels_config
        .read()
        .await
        .load_label(&label_id)
        .await?;

    Ok(content.into_response(&request))
}
