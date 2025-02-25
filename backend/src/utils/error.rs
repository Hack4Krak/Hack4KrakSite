use actix_web::http::StatusCode;
use actix_web::{HttpResponse, error};
use serde_json::json;
use thiserror::Error;

use crate::entities::sea_orm_active_enums::UserRoles;
use crate::routes::auth::AuthError;
use crate::routes::flag::FlagError;
use crate::routes::task::TaskError;
use crate::routes::teams::TeamError;

pub struct ErrorHttpResponseExtension {
    pub error: String,
}

impl ErrorHttpResponseExtension {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

pub fn error_response_builder<T: error::ResponseError>(err: &T) -> HttpResponse {
    let status_code = err.status_code();
    let error_message = err.to_string();

    let mut response = HttpResponse::build(status_code).json(json!({
        "code": status_code.as_u16(),
        "message": error_message,
        "error": format!("{:?}", err),
    }));

    response
        .extensions_mut()
        .insert(ErrorHttpResponseExtension::new(error_message));

    response
}

#[derive(Debug, Error)]
pub enum Error {
    #[error("Failed to hash password: {0}")]
    HashPasswordFailed(argon2::password_hash::Error),
    #[error("Failed to parse response from OAuth provider")]
    OAuth,
    #[error("Request error: {0}")]
    Request(#[from] reqwest::Error),
    #[error("Database operation failed")]
    DatabaseOperation(#[from] sea_orm::DbErr),
    #[error("Unauthorized")]
    Unauthorized,
    #[error("Thou shall not pass, required role: {required_role:?}")]
    Forbidden { required_role: UserRoles },
    #[error("Invalid Json Web Token")]
    InvalidJsonWebToken,
    #[error("Invalid authorization header content")]
    InvalidAuthorizationHeader,
    #[error("Lock is poisoned")]
    PoisonedLock,
    #[error("IO Error")]
    Io(#[from] std::io::Error),
    #[error("User not found")]
    UserNotFound,
    #[error("Missing {name} extension in request")]
    MissingExtension { name: String },
    #[error("Placeholder elements are required for this email template")]
    PlaceholdersRequired,
    #[error("Failed to send email: {0}")]
    FailedToSendEmail(#[from] lettre::transport::smtp::Error),
    #[error("Failed to build email: {0}")]
    FailedToBuildEmail(#[from] lettre::error::Error),
    #[error("Could not serialize json: {0}")]
    InvalidJson(#[from] serde_json::Error),
    #[error("Route not found")]
    RouteNotFound,
    #[error("Invalid sender's email {0}")]
    InvalidEmailSender(String),
    #[error("Invalid recipients' email {0}")]
    InvalidEmailRecipients(String),
    #[error("User must have higher role than the affected user")]
    UserMustHaveHigherRoleThanAffectedUser,
    #[error("User with such email or username already exists")]
    UserWithEmailOrUsernameAlreadyExists,
    #[error("You cannot access this endpoint before our event has started")]
    AccessBeforeEventStart,
    #[error("You cannot access this endpoint after our event has finished")]
    AccessAfterEventEnd,
    #[error("Recipient not found {username}")]
    RecipientNotFound { username: String },
    #[error("You cannot access this endpoint after during event")]
    AccessDuringEvent,
    #[error("Failed to parse URL {0}")]
    FailedToParseUrl(#[from] url::ParseError),
    #[error(transparent)]
    Auth(#[from] AuthError),
    #[error(transparent)]
    Team(#[from] TeamError),
    #[error(transparent)]
    Task(#[from] TaskError),
    #[error(transparent)]
    Flag(#[from] FlagError),
}

impl error::ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::PlaceholdersRequired
            | Error::InvalidEmailSender(_)
            | Error::InvalidEmailRecipients(_) => StatusCode::BAD_REQUEST,
            Error::HashPasswordFailed(_)
            | Error::DatabaseOperation(_)
            | Error::OAuth
            | Error::PoisonedLock
            | Error::Io(_)
            | Error::Request(_)
            | Error::FailedToSendEmail(_)
            | Error::InvalidJson(_)
            | Error::FailedToBuildEmail(_)
            | Error::FailedToParseUrl(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::InvalidJsonWebToken => StatusCode::UNAUTHORIZED,
            Error::InvalidAuthorizationHeader | Error::MissingExtension { .. } => {
                StatusCode::BAD_REQUEST
            }
            Error::UserNotFound | Error::RouteNotFound | Error::RecipientNotFound { .. } => {
                StatusCode::NOT_FOUND
            }
            Error::Forbidden { .. }
            | Error::UserMustHaveHigherRoleThanAffectedUser
            | Error::AccessDuringEvent
            | Error::AccessBeforeEventStart => StatusCode::FORBIDDEN,
            Error::UserWithEmailOrUsernameAlreadyExists => StatusCode::CONFLICT,
            Error::AccessAfterEventEnd => StatusCode::GONE,
            Error::Team(team_err) => team_err.status_code(),
            Error::Auth(auth_err) => auth_err.status_code(),
            Error::Task(error) => error.status_code(),
            Error::Flag(error) => error.status_code(),
        }
    }

    fn error_response(&self) -> HttpResponse {
        error_response_builder(self)
    }
}
