use actix_web::{post, HttpResponse};

use crate::utils::cookies::{reset_cookie, ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE};
use crate::utils::error::Error;

#[utoipa::path(
    responses(
        (status = 200, description = "User successfully logged out.")
    ),
    tag = "auth"
)]
#[post("/logout")]
pub async fn logout() -> Result<HttpResponse, Error> {
    Ok(HttpResponse::Ok()
        .append_header(("Set-Cookie", reset_cookie(ACCESS_TOKEN_COOKIE)))
        .append_header(("Set-Cookie", reset_cookie(REFRESH_TOKEN_COOKIE)))
        .finish())
}
