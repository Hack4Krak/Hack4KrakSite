use actix_web::middleware::Next;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    Error as ActixError, HttpMessage,
};

use crate::utils::cookies::ACCESS_TOKEN_COOKIE;
use crate::utils::error::Error;
use crate::utils::jwt::decode_jwt;

pub async fn check_auth_middleware(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixError> {
    let Some(cookie) = request.cookie(ACCESS_TOKEN_COOKIE) else {
        return Err(ActixError::from(Error::Unauthorized));
    };

    let claims = decode_jwt(cookie.value()).map_err(|_| Error::Unauthorized)?;
    request.extensions_mut().insert(claims.claims);

    next.call(request).await
}
