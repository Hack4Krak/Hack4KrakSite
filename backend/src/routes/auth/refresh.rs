use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::routes::auth::TokensResponse;
use crate::utils::error::Error;
use crate::utils::jwt::{decode_jwt, get_default_tokens};

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[utoipa::path(
    request_body = RefreshToken,
    responses(
        (status = 200, description = "New tokens received", body = TokensResponse),
        (status = 401, description = "Invalid credentials."),
    ),
    tag = "auth"
)]
#[post("/refresh")]
pub async fn refresh(data: web::Json<RefreshToken>) -> Result<HttpResponse, Error> {
    let claim = decode_jwt(&data.refresh_token).map_err(|_| Error::Unauthorized)?;
    let email = claim.claims.email;
    let tokens = get_default_tokens(email)?;

    Ok(HttpResponse::Ok().json(tokens))
}
