use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse, HttpResponseBuilder};
use thiserror::Error;
use utoipa::gen::serde_json::json;

use crate::routes::auth::AuthError;
use crate::routes::teams::TeamError;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to hash password: {0}")]
    HashPasswordFailed(argon2::password_hash::Error),
    #[error("Failed to proceed with OAuth flow")]
    OAuth,
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Database operation failed")]
    DatabaseOperation(#[from] sea_orm::DbErr),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Invalid Json Web Token")]
    InvalidJsonWebToken,
    #[error("Invalid authorization header content")]
    InvalidAuthorizationHeader,
    #[error("Placeholder elements are required for this email template")]
    PlaceholdersRequired,
    #[error("Failed to send email: {0}")]
    FailedToSendEmail(#[from] lettre::transport::smtp::Error),
    #[error("Email template not found")]
    EmailTemplateNotFound,
    #[error("Invalid email address while sending email")]
    InvalidEmailAddressSendingEmail,
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error(transparent)]
    Team(#[from] TeamError),
}

pub fn json_error_response<T: error::ResponseError>(err: &T) -> HttpResponse {
    HttpResponseBuilder::new(err.status_code()).json(json!({
        "code": err.status_code().as_u16(),
        "message": err.to_string(),
        "error": format!("{:?}", err),
    }))
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::PlaceholdersRequired
            | Error::EmailTemplateNotFound
            | Error::InvalidEmailAddressSendingEmail => StatusCode::BAD_REQUEST,
            Error::HashPasswordFailed(_)
            | Error::DatabaseOperation(_)
            | Error::OAuth
            | Error::Request(_)
            | Error::FailedToSendEmail(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized
            | Error::InvalidJsonWebToken
            | Error::InvalidAuthorizationHeader => StatusCode::UNAUTHORIZED,
            Error::Team(team_err) => team_err.status_code(),
            Error::Auth(auth_err) => auth_err.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        json_error_response(self)
    }
}
