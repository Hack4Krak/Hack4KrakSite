use crate::entities::{email_verification_request, users};
use crate::models::email_verification_request::EmailVerificationAction;
use crate::models::user::{Password, UserInformation};
use crate::routes::auth::AuthError::{
    InvalidCredentials, InvalidEmailAddress, PasswordAuthNotAvailable,
};
use crate::routes::auth::RegisterModel;
use crate::routes::auth::reset_password::ResetPasswordModel;
use crate::services::emails;
use crate::services::emails::EmailConfirmation;
use crate::services::env::EnvConfig;
use crate::utils::app_state;
use crate::utils::cookies::{
    ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE, create_cookie, reset_cookie,
};
use crate::utils::email::Email;
use crate::utils::error::Error;
use crate::utils::error::Error::HashPasswordFailed;
use crate::utils::jwt::encode_jwt;
use crate::utils::success_response::SuccessResponse;
use actix_web::{HttpResponse, HttpResponseBuilder};
use argon2::password_hash::SaltString;
use argon2::password_hash::rand_core::OsRng;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Duration;
use uuid::Uuid;
use validator::ValidateEmail;

pub struct AuthService;

impl AuthService {
    pub async fn register_with_password(
        app_state: &app_state::AppState,
        credentials: RegisterModel,
    ) -> Result<HttpResponse, Error> {
        if !credentials.email.validate_email() {
            return Err(Error::Auth(InvalidEmailAddress));
        }

        let password_hash = Self::hash_password(credentials.password.clone())?;
        let user_information =
            UserInformation::new(&app_state.database, password_hash, &credentials).await?;

        let confirmation_code = email_verification_request::Model::create(
            &app_state.database,
            EmailVerificationAction::ConfirmEmailAddress { user_information },
            credentials.email.clone(),
            Some(Duration::minutes(30)),
        )
        .await?;

        let link = Self::create_email_confirmation_link(&confirmation_code.to_string())?;

        Email::new(
            "auth",
            vec![credentials.email],
            Box::new(EmailConfirmation {
                link,
                user: credentials.name,
            }),
        )
        .send(&app_state.smtp_client)
        .await?;

        Ok(SuccessResponse::default().http_response())
    }

    pub fn assert_password_is_valid(
        user: &users::Model,
        password_to_verify: &Password,
    ) -> Result<(), Error> {
        let password = user
            .password
            .clone()
            .ok_or(Error::Auth(PasswordAuthNotAvailable))?;

        let parsed_hash = PasswordHash::new(&password).map_err(Error::HashPasswordFailed)?;

        let is_verified = Argon2::default()
            .verify_password(password_to_verify.0.as_bytes(), &parsed_hash)
            .is_ok();

        if !is_verified {
            return Err(Error::Auth(InvalidCredentials));
        }

        Ok(())
    }

    pub fn append_tokens_as_cookies(
        uuid: Uuid,
        email: String,
        http_response: &mut HttpResponseBuilder,
    ) -> Result<(), Error> {
        let access_token = encode_jwt(uuid, email.clone(), Duration::minutes(10))?;
        let refresh_token = encode_jwt(uuid, email, Duration::days(14))?;

        let refresh_cookie = create_cookie(
            REFRESH_TOKEN_COOKIE,
            &refresh_token,
            Some(actix_web::cookie::time::Duration::days(14)),
        );
        let access_cookie = create_cookie(ACCESS_TOKEN_COOKIE, &access_token, None);

        http_response.append_header(("Set-Cookie", refresh_cookie));
        http_response.append_header(("Set-Cookie", access_cookie));

        Ok(())
    }

    pub fn response_with_cookies(uuid: Uuid, email: String) -> Result<HttpResponse, Error> {
        let mut response = HttpResponse::Ok();
        Self::append_tokens_as_cookies(uuid, email, &mut response)?;
        Ok(response.finish())
    }

    pub async fn confirm_email(
        app_state: &app_state::AppState,
        confirmation_code: Uuid,
    ) -> Result<(), Error> {
        let email_confirmation = email_verification_request::Model::find_and_verify(
            &app_state.database,
            confirmation_code,
        )
        .await?;
        let EmailVerificationAction::ConfirmEmailAddress { user_information } =
            email_confirmation.get_action()?
        else {
            return Err(Error::InvalidEmailConfirmationCode);
        };

        users::Model::create_from_user_info(&app_state.database, user_information).await?;

        email_confirmation.delete(&app_state.database).await?;

        Ok(())
    }

    fn create_email_confirmation_link(confirmation_code: &str) -> Result<String, Error> {
        let mut url = EnvConfig::get().backend_url.clone();
        url = url.join("/auth/confirm/")?;
        url = url.join(confirmation_code)?;

        Ok(url.to_string())
    }

    pub fn hash_password(password: Password) -> Result<String, Error> {
        let salt = SaltString::try_from_rng(&mut OsRng)?;

        Ok(Argon2::default()
            .hash_password(password.0.as_bytes(), &salt)
            .map_err(HashPasswordFailed)?
            .to_string())
    }

    pub async fn request_password_reset(
        app_state: &app_state::AppState,
        email: String,
    ) -> Result<(), Error> {
        users::Model::find_by_email(&app_state.database, &email)
            .await?
            .ok_or(Error::Auth(InvalidEmailAddress))?;

        let confirmation_code = email_verification_request::Model::create(
            &app_state.database,
            EmailVerificationAction::ResetPassword,
            email.clone(),
            Some(Duration::minutes(30)),
        )
        .await?
        .to_string();

        let mut reset_password_link = EnvConfig::get().frontend_url.clone();
        reset_password_link = reset_password_link
            .join(format!("/reset_password?code={confirmation_code}").as_str())?;

        Email::new(
            "auth",
            vec![email],
            Box::new(emails::ResetPassword {
                link: reset_password_link.to_string(),
            }),
        )
        .send(&app_state.smtp_client)
        .await?;

        Ok(())
    }

    pub async fn reset_password(
        app_state: &app_state::AppState,
        model: ResetPasswordModel,
    ) -> Result<(), Error> {
        let password_reset =
            email_verification_request::Model::find_and_verify(&app_state.database, model.code)
                .await?;
        let EmailVerificationAction::ResetPassword = password_reset.get_action()? else {
            return Err(Error::InvalidEmailConfirmationCode);
        };

        let user = users::Model::find_by_email(&app_state.database, &password_reset.email)
            .await?
            .ok_or(Error::Auth(InvalidEmailAddress))?;

        users::Model::update(
            &app_state.database,
            user,
            users::UpdatableModel {
                password: Some(Some(Self::hash_password(model.new_password)?)),
                ..Default::default()
            },
        )
        .await?;

        password_reset.delete(&app_state.database).await?;

        Ok(())
    }

    pub fn reset_cookies_response() -> HttpResponse {
        HttpResponse::Ok()
            .append_header(("Set-Cookie", reset_cookie(ACCESS_TOKEN_COOKIE)))
            .append_header(("Set-Cookie", reset_cookie(REFRESH_TOKEN_COOKIE)))
            .finish()
    }
}
