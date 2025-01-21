use actix_web::http::StatusCode;
use actix_web::{error, HttpResponse, HttpResponseBuilder};
use thiserror::Error;
use utoipa::gen::serde_json::json;

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
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid username and/or password")]
    InvalidCredentials,
    #[error("Invalid email address")]
    InvalidEmailAddress,
    #[error("Password & email authentication is not available for this account")]
    PasswordAuthNotAvailable,
    #[error("Invalid Json Web Token")]
    InvalidJsonWebToken,
    #[error("Invalid authorization header content")]
    InvalidAuthorizationHeader,
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Team already exists")]
    TeamAlreadyExists,
    #[error("User already belongs to team: {team_name}")]
    UserAlreadyBelongsToTeam { team_name: String },
    #[error("Team not found")]
    TeamNotFound,
    #[error("User doesn't belong to any team")]
    UserDoesntBelongToAnyTeam,
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match *self {
            // 4xx: Client Errors
            Error::InvalidEmailAddress => StatusCode::BAD_REQUEST,
            Error::InvalidAuthorizationHeader => StatusCode::UNAUTHORIZED,
            Error::InvalidCredentials | Error::InvalidJsonWebToken => StatusCode::UNAUTHORIZED,
            Error::Unauthorized
            | Error::UserAlreadyBelongsToTeam { team_name: _ }
            | Error::PasswordAuthNotAvailable
            | Error::UserDoesntBelongToAnyTeam => StatusCode::FORBIDDEN,
            Error::UserAlreadyExists | Error::TeamAlreadyExists => StatusCode::CONFLICT,
            Error::TeamNotFound => StatusCode::NOT_FOUND,
            // 5xx: Server Errors
            Error::HashPasswordFailed(_)
            | Error::DatabaseOperation(_)
            | Error::OAuth
            | Error::Request(_) => StatusCode::INTERNAL_SERVER_ERROR,
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
