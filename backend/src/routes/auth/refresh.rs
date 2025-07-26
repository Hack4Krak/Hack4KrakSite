use crate::services::auth::AuthService;
use crate::utils::cookies::REFRESH_TOKEN_COOKIE;
use crate::utils::error::Error;
use actix_web::{HttpRequest, HttpResponse, post};

use crate::utils::jwt::decode_jwt;

#[utoipa::path(
    responses(
        (status = 200, description = "New tokens are set as cookies"),
        (status = 401, description = "Invalid credentials."),
    ),
    security(
        ("access_token" = [])
    ),
    tag = "auth"
)]
#[post("/refresh")]
pub async fn refresh(request: HttpRequest) -> Result<HttpResponse, Error> {
    let Some(refresh_token) = request.cookie(REFRESH_TOKEN_COOKIE) else {
        return Err(Error::Unauthorized);
    };

    // todo: WE DO NOT CHECK IF USER EXISTS

    let claims = decode_jwt(refresh_token.value()).map_err(|_| Error::Unauthorized)?;
    let response = AuthService::response_with_cookies(claims.claims.id, claims.claims.email)?;

    Ok(response)
}
