use crate::utils::cookies::REFRESH_TOKEN_COOKIE;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::{HttpRequest, HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "User is logged in"),
        (status = 401, description = "User is not logged in."),
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "auth-status",
    tag = "auth"
)]
#[get("/status")]
pub async fn status(request: HttpRequest) -> Result<HttpResponse, Error> {
    if request.cookie(REFRESH_TOKEN_COOKIE).is_some() {
        Ok(SuccessResponse::default().http_response())
    } else {
        Err(Error::Unauthorized)
    }
}
