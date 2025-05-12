use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "Use information received."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "admin-index",
    tag = "admin"
)]
#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(SuccessResponse::default().http_response())
}
