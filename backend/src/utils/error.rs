use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse};
use thiserror::Error;
use utoipa::gen::serde_json::json;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to hash password: {0}")]
    HashPasswordFailed(argon2::password_hash::Error),
    #[error("Database operation failed")]
    DatabaseOperation(#[from] sea_orm::DbErr),
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid username and/or password")]
    InvalidCredentials,
    #[error("Invalid email address")]
    InvalidEmailAddress,
    #[error("Invalid Json Web Token")]
    InvalidJsonWebToken,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("User doesn't exist")]
    UserDoesNotExist,
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            Error::UserAlreadyExists => StatusCode::BAD_REQUEST,
            Error::InvalidEmailAddress => StatusCode::BAD_REQUEST,
            Error::InvalidCredentials => StatusCode::UNAUTHORIZED,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::InvalidJsonWebToken => StatusCode::INTERNAL_SERVER_ERROR,
            Error::HashPasswordFailed(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::DatabaseOperation(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::UserDoesNotExist => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::BadRequest().json(json!({
            "code": self.status_code().as_u16(),
            "message": self.to_string(),
            "error": format!("{:?}", self),
        }))
    }
}
