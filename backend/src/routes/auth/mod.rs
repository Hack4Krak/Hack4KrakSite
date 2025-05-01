mod confirm;
mod login;
mod logout;
mod oauth;
mod refresh;
mod register;
pub mod reset_password;

use crate::utils::error::error_response_builder;
use actix_web::error;
use actix_web::http::StatusCode;
use hack4krak_macros::error_with_messages;
pub use register::RegisterModel;

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
    config.service(reset_password::request_reset_password);
    config.service(reset_password::reset_password);
}

#[error_with_messages]
pub enum AuthError {
    UserAlreadyExists,
    InvalidCredentials,
    InvalidEmailAddress,
    PasswordAuthNotAvailable,
}

impl error::ResponseError for AuthError {
    fn status_code(&self) -> StatusCode {
        match self {
            AuthError::UserAlreadyExists => StatusCode::CONFLICT,
            AuthError::InvalidCredentials
            | AuthError::InvalidEmailAddress
            | AuthError::PasswordAuthNotAvailable => StatusCode::UNAUTHORIZED,
        }
    }

    fn error_response(&self) -> actix_web::HttpResponse {
        error_response_builder(self)
    }
}
