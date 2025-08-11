use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::http::header;
use actix_web::http::header::DispositionType;
use actix_web::{HttpRequest, HttpResponse, get, web};

#[utoipa::path(
    responses(
        (status = 200, description = "List of task assets.", body = String),
        (status = 404, description = "Asset does not exist."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "task/assets"
)]
#[get("/get/{task_id}/{asset_path}")]
pub async fn get(
    app_data: web::Data<AppState>,
    request: HttpRequest,
    task_asset: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let manager = &app_data.task_manager;

    let (task_id, asset_path) = task_asset.into_inner();

    let asset_path = "assets/".to_owned() + asset_path.as_str();

    let named_file = manager.load_asset(&task_id, &asset_path).await?;

    Ok(named_file
        .set_content_disposition(header::ContentDisposition {
            disposition: DispositionType::Attachment,
            parameters: Vec::new(),
        })
        .into_response(&request))
}
