use crate::utils::error::Error;
use crate::utils::jwt::{decode_jwt, get_default_tokens};
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RefreshToken {
    pub refresh_token: String,
}

#[utoipa::path(
    post,
    path = "/refresh",
    request_body = RefreshToken,
    responses(
        (status = 200, description = "New tokens received"),
        (status = 401, description = "Invalid credentials."),
    )
)]
#[post("/refresh")]
pub async fn refresh(data: web::Json<RefreshToken>) -> Result<HttpResponse, Error> {
    let claim = decode_jwt(&data.refresh_token).map_err(|_| Error::Unauthorized)?;
    let email = claim.claims.email;
    let tokens = get_default_tokens(email)?;

    Ok(HttpResponse::Ok().json(tokens))
}
