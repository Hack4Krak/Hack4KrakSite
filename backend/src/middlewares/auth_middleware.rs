use actix_web::body::BoxBody;
use actix_web::middleware::Next;
use actix_web::{
    dev::{ServiceRequest, ServiceResponse},
    Error as ActixError, HttpMessage, ResponseError,
};

use crate::utils::cookies::ACCESS_TOKEN_COOKIE;
use crate::utils::error::Error;
use crate::utils::jwt::decode_jwt;

pub async fn check_auth_middleware(
    request: ServiceRequest,
    next: Next<BoxBody>,
) -> Result<ServiceResponse<BoxBody>, ActixError> {
    let Some(cookie) = request.cookie(ACCESS_TOKEN_COOKIE) else {
        return Ok(request.into_response(Error::Unauthorized.error_response()));
    };

    let Ok(claims) = decode_jwt(cookie.value()) else {
        return Ok(request.into_response(Error::Unauthorized.error_response()));
    };

    request.extensions_mut().insert(claims.claims);
    next.call(request).await
}
