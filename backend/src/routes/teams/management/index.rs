use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{get, HttpResponse};

#[utoipa::path(
    responses(
        (status = 200, description = "User is team leader."),
        (status = 500, description = "Internal server error."),
    ),
    security(
        ("access_token" = ["leader"])
    ),
    tag = "teams/management"
)]
#[get("/")]
pub async fn index() -> Result<HttpResponse, Error> {
    Ok(SuccessResponse::default().http_response())
}
