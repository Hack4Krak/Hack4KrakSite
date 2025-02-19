use crate::services::env::EnvConfig;
use crate::utils::error::Error;
use actix_files::NamedFile;
use actix_web::{get, web, HttpRequest, HttpResponse};

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
    request: HttpRequest,
    task_asset: web::Path<(String, String)>,
) -> Result<HttpResponse, Error> {
    let task_asset = task_asset.into_inner();

    let (task_id, asset_path) = (task_asset.0, task_asset.1);

    let asset_path_full = EnvConfig::get()
        .tasks_base_path
        .clone()
        .join(task_id)
        .join("assets/")
        .join(asset_path);

    let stream = NamedFile::open(asset_path_full)?.into_response(&request);

    Ok(stream)
}
