use crate::utils::error::Error;
use crate::utils::jwt::decode_jwt;
use actix_web::middleware::Next;
use actix_web::{
    body::MessageBody,
    dev::{ServiceRequest, ServiceResponse},
    http::header::AUTHORIZATION,
    Error as ActixError, HttpMessage,
};

pub async fn check_auth_middleware(
    request: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixError> {
    let auth_header = request
        .headers()
        .get(AUTHORIZATION)
        .ok_or(ActixError::from(Error::Unauthorized))?;

    let token = auth_header
        .to_str()
        .map_err(|_| Error::InvalidAuthorizationHeader)?
        .trim_start_matches("Bearer ");

    let claims = decode_jwt(token).map_err(|_| Error::Unauthorized)?;
    request.extensions_mut().insert(claims.claims);

    next.call(request).await
}
