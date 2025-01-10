use actix_web::body::BoxBody;
use actix_web::{error, HttpResponse};
use thiserror::Error;
use utoipa::gen::serde_json::json;

#[derive(Debug, Error)]
pub enum Error {}

impl error::ResponseError for Error {
    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::BadRequest().json(json!({
            "code": self.status_code().as_u16(),
            "message": self.to_string(),
            "error": format!("{:?}", self),
        }))
    }
}
