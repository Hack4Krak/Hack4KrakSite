use crate::entities::sea_orm_active_enums::UserRoles;
use crate::routes::account::AccountError;
use crate::routes::auth::AuthError;
use crate::routes::flag::FlagError;
use crate::routes::task::TaskError;
use crate::routes::teams::TeamError;
use actix_web::http::StatusCode;
use actix_web::{HttpRequest, HttpResponse, error};
use hack4krak_macros::error_with_messages;
use rand::rand_core::OsError;
use sea_orm::RuntimeErr::SqlxError;
use serde::Serialize;
use serde_json::{Value, json, to_value};
use tokio::sync::broadcast;

pub struct ErrorHttpResponseExtension {
    pub error: String,
}

impl ErrorHttpResponseExtension {
    pub fn new(error: String) -> Self {
        Self { error }
    }
}

pub fn error_response_builder<T: error::ResponseError + Serialize>(error: &T) -> HttpResponse {
    let status_code = error.status_code();
    let error_message = error.to_string();

    let mut data: Value = to_value(error).unwrap_or(json!({}));
    if let Value::Object(map) = &mut data {
        map.insert("code".to_string(), json!(status_code.as_u16()));
        map.insert("message".to_string(), json!(error_message));

        if !map.contains_key("error") {
            map.insert("error".to_string(), json!(format!("{:?}", error)));
        }
    }

    let mut response = HttpResponse::build(status_code).json(data);
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
    #[serde(skip)]
    HashPasswordFailed(argon2::password_hash::Error),
    OAuth,
    #[serde(skip)]
    Request(#[from] reqwest::Error),
    #[serde(skip)]
    DatabaseOperation(sea_orm::DbErr),
    ConflictInDatabase,
    Unauthorized,
    Forbidden {
        required_role: UserRoles,
    },
    InvalidJsonWebToken,
    #[serde(skip)]
    Io(#[from] std::io::Error),
    UserNotFound,
    MissingExtension {
        name: String,
    },
    PlaceholdersRequired,
    #[serde(skip)]
    FailedToSendEmail(#[from] lettre::transport::smtp::Error),
    #[serde(skip)]
    FailedToBuildEmail(#[from] lettre::error::Error),
    #[serde(skip)]
    InvalidJson(#[from] serde_json::Error),
    InvalidEmailConfirmationCode,
    InvalidColorFormat,
    EmailConfirmationCodeExpired,
    RouteNotFound,
    InvalidEmailSender(String),
    InvalidEmailRecipients(String),
    UserMustHaveHigherRoleThanAffectedUser,
    UserMustBeOwnerToUpdateRoles,
    UserWithEmailOrUsernameAlreadyExists,
    AccessBeforeEventStart {
        event_start_date: String,
    },
    AccessAfterEventEnd,
    RecipientNotFound {
        username: String,
    },
    AccessDuringEvent,
    #[serde(skip)]
    FailedToParseUrl(#[from] url::ParseError),
    #[serde(skip)]
    ServerEventSendingError(#[from] broadcast::error::SendError<String>),
    #[serde(skip)]
    Metrics(#[from] prometheus::Error),
    #[serde(skip)]
    Os(#[from] OsError),
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
            | Error::Metrics(_)
            | Error::Os(_)
            | Error::ServerEventSendingError(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::InvalidJsonWebToken => StatusCode::UNAUTHORIZED,
            Error::MissingExtension { .. }
            | Error::InvalidEmailConfirmationCode
            | Error::EmailConfirmationCodeExpired
            | Error::InvalidColorFormat => StatusCode::BAD_REQUEST,
            Error::UserNotFound | Error::RouteNotFound | Error::RecipientNotFound { .. } => {
                StatusCode::NOT_FOUND
            }
            Error::Forbidden { .. }
            | Error::UserMustHaveHigherRoleThanAffectedUser
            | Error::AccessDuringEvent
            | Error::UserMustBeOwnerToUpdateRoles
            | Error::AccessBeforeEventStart { .. } => StatusCode::FORBIDDEN,
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
