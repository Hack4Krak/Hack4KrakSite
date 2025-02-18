use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Bytes;
use actix_web::{get, web, HttpResponse};
use futures_util::future::ok;
use futures_util::stream::once;

#[utoipa::path(
    responses(
        (status = 200, description = "List of task assets."),
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
    app_state: web::Data<app_state::AppState>,
    task_asset: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let manager = &app_state.task_manager;

    let task_asset = task_asset.into_inner();

    let mut asset_path_full = "assets/".to_string();
    asset_path_full.push_str(&task_asset.1);

    let content = manager.load_asset(&task_asset.0, &asset_path_full).await?;

    let body = once(ok::<_, Error>(Bytes::from(content)));

    Ok(HttpResponse::Ok()
        .content_type("application/octet-stream")
        .streaming(body))
}
