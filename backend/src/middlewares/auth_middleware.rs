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
    req: ServiceRequest,
    next: Next<impl MessageBody>,
) -> Result<ServiceResponse<impl MessageBody>, ActixError> {
    let auth = req.headers().get(AUTHORIZATION);

    if auth.is_none() {
        return Err(ActixError::from(Error::Unauthorized));
    }

    let token = auth
        .unwrap()
        .to_str()
        .unwrap()
        .replace("Bearer ", "")
        .to_owned();
    let claim = decode_jwt(&token).map_err(|_| Error::Unauthorized)?;
    req.extensions_mut().insert(claim.claims);

    next.call(req).await
}
