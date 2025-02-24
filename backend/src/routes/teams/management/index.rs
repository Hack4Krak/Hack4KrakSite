use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "User is team leader."),
        (status = 500, description = "Internal server error."),
    ),
    security(
        ("access_token" = ["leader"])
    ),
    operation_id = "teams_management_index",
    tag = "teams/management"
)]
#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(SuccessResponse::default().http_response())
}
