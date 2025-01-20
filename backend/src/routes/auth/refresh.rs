use actix_web::{post, HttpRequest, HttpResponse};

use crate::utils::cookies::REFRESH_TOKEN_COOKIE;
use crate::utils::error::Error;
use crate::utils::jwt::{decode_jwt, get_tokens_http_response};

#[utoipa::path(
    responses(
        (status = 200, description = "New tokens are set as cookies"),
        (status = 401, description = "Invalid credentials."),
    ),
    tag = "auth"
)]
#[post("/refresh")]
pub async fn refresh(request: HttpRequest) -> Result<HttpResponse, Error> {
    let Some(refresh_token) = request.cookie(REFRESH_TOKEN_COOKIE) else {
        return Err(Error::Unauthorized);
    };

    let claim = decode_jwt(refresh_token.value()).map_err(|_| Error::Unauthorized)?;
    let email = claim.claims.email;
    let response = get_tokens_http_response(email)?;

    Ok(response)
}
