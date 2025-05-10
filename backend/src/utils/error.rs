use crate::entities::sea_orm_active_enums::UserRoles;
use crate::routes::account::AccountError;
use crate::routes::auth::AuthError;
use crate::routes::flag::FlagError;
use crate::routes::task::TaskError;
use crate::routes::teams::TeamError;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, error};
use hack4krak_macros::error_with_messages;
use sea_orm::RuntimeErr::SqlxError;
use serde_json::json;
use tokio::sync::broadcast;

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

pub fn validation_error_handler(
    errors: validator::ValidationErrors,
    _: &HttpRequest,
) -> actix_web::Error {
    Error::Validator(errors).into()
}

#[error_with_messages]
pub enum Error {
    HashPasswordFailed(argon2::password_hash::Error),
    OAuth,
    Request(#[from] reqwest::Error),
    DatabaseOperation(sea_orm::DbErr),
    ConflictInDatabase,
    Unauthorized,
    Forbidden {
        required_role: UserRoles,
    },
    InvalidJsonWebToken,
    Io(#[from] std::io::Error),
    UserNotFound,
    MissingExtension {
        name: String,
    },
    PlaceholdersRequired,
    FailedToSendEmail(#[from] lettre::transport::smtp::Error),
    FailedToBuildEmail(#[from] lettre::error::Error),
    InvalidJson(#[from] serde_json::Error),
    InvalidEmailConfirmationCode,
    EmailConfirmationCodeExpired,
    RouteNotFound,
    InvalidEmailSender(String),
    InvalidEmailRecipients(String),
    UserMustHaveHigherRoleThanAffectedUser,
    UserMustBeOwnerToUpdateRoles,
    UserWithEmailOrUsernameAlreadyExists,
    AccessBeforeEventStart,
    AccessAfterEventEnd,
    RecipientNotFound {
        username: String,
    },
    AccessDuringEvent,
    FailedToParseUrl(#[from] url::ParseError),
    ServerEventSendingError(#[from] broadcast::error::SendError<String>),
    Validator(validator::ValidationErrors),

    #[error(transparent)]
    Account(#[from] AccountError),
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
            | Error::Validator(_)
            | Error::InvalidEmailRecipients(_) => StatusCode::BAD_REQUEST,
            Error::HashPasswordFailed(_)
            | Error::DatabaseOperation(_)
            | Error::OAuth
            | Error::Io(_)
            | Error::Request(_)
            | Error::FailedToSendEmail(_)
            | Error::InvalidJson(_)
            | Error::FailedToBuildEmail(_)
            | Error::FailedToParseUrl(_)
            | Error::ConflictInDatabase
            | Error::ServerEventSendingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::InvalidJsonWebToken => StatusCode::UNAUTHORIZED,
            Error::MissingExtension { .. }
            | Error::InvalidEmailConfirmationCode
            | Error::EmailConfirmationCodeExpired => StatusCode::BAD_REQUEST,
            Error::UserNotFound | Error::RouteNotFound | Error::RecipientNotFound { .. } => {
                StatusCode::NOT_FOUND
            }
            Error::Forbidden { .. }
            | Error::UserMustHaveHigherRoleThanAffectedUser
            | Error::AccessDuringEvent
            | Error::UserMustBeOwnerToUpdateRoles
            | Error::AccessBeforeEventStart => StatusCode::FORBIDDEN,
            Error::UserWithEmailOrUsernameAlreadyExists => StatusCode::CONFLICT,
            Error::AccessAfterEventEnd => StatusCode::GONE,
            Error::Account(account_err) => account_err.status_code(),
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

impl From<sea_orm::DbErr> for Error {
    fn from(value: sea_orm::DbErr) -> Self {
        match &value {
            sea_orm::DbErr::Query(SqlxError(sqlx_error)) => {
                if let Some(pg_err) = sqlx_error.as_database_error() {
                    if pg_err.is_unique_violation() {
                        return Error::ConflictInDatabase;
                    }
                }
                Error::DatabaseOperation(value)
            }
            _ => Error::DatabaseOperation(value),
        }
    }
}
