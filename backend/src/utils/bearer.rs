use crate::utils::error::Error;
use actix_http::header::AUTHORIZATION;
use actix_web::HttpRequest;

pub fn verify_bearer_token(req: &HttpRequest, expected_token: &str) -> Result<(), Error> {
    let header_value = req
        .headers()
        .get(AUTHORIZATION)
        .and_then(|hv| hv.to_str().ok());

    match header_value {
        Some(header) if header == format!("Bearer {}", expected_token) => Ok(()),
        _ => Err(Error::Unauthorized),
    }
}
