mod login;
mod oauth;
mod refresh;
mod register;

use crate::utils::error::json_error_response;
use actix_web::error;
use actix_web::http::StatusCode;
pub use login::LoginModel;
pub use login::TokensResponse;
pub use register::RegisterModel;
use thiserror::Error;

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(register::register);
    cfg.service(login::login);
    cfg.service(refresh::refresh);
    cfg.service(oauth::github::github);
    cfg.service(oauth::github::github_callback);
    cfg.service(oauth::google::google_callback);
    cfg.service(oauth::google::google);
}

#[derive(Debug, Error)]
pub enum AuthError {
    #[error("User already exists")]
    UserAlreadyExists,
    #[error("Invalid username and/or password")]
    InvalidCredentials,
    #[error("Invalid email address")]
    InvalidEmailAddress,
    #[error("Password & email authentication is not available for this account")]
    PasswordAuthNotAvailable,
}

impl error::ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthError::InvalidCredentials
            | AuthError::InvalidEmailAddress
            | AuthError::PasswordAuthNotAvailable => StatusCode::FORBIDDEN,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        json_error_response(self)
    }
}
