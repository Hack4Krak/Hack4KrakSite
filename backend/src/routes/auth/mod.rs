mod confirm;
mod login;
mod logout;
mod oauth;
mod refresh;
mod register;

use crate::utils::error::error_response_builder;
use actix_web::error;
use actix_web::http::StatusCode;
pub use login::LoginModel;
pub use register::RegisterModel;
use thiserror::Error;

pub fn config(config: &mut utoipa_actix_web::service_config::ServiceConfig) {
    config.service(register::register);
    config.service(login::login);
    config.service(logout::logout);
    config.service(refresh::refresh);
    config.service(oauth::github::github);
    config.service(oauth::github::github_callback);
    config.service(oauth::google::google_callback);
    config.service(oauth::google::google);
    config.service(confirm::confirm_email);
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
    #[error("Invalid confirmation code")]
    InvalidConfirmationCode,
    #[error("Confirmation code expired, please try sending email again")]
    ConfirmationCodeExpired,
}

impl error::ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthError::InvalidCredentials
            | AuthError::InvalidEmailAddress
            | AuthError::PasswordAuthNotAvailable
            | AuthError::InvalidConfirmationCode
            | AuthError::ConfirmationCodeExpired => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        error_response_builder(self)
    }
}
