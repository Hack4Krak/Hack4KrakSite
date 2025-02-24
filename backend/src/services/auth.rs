use crate::entities::{email_confirmation, users};
use crate::models::user::UserInformation;
use crate::routes::account::update::UpdateUserModel;
use crate::routes::auth::AuthError::{
    InvalidCredentials, InvalidEmailAddress, PasswordAuthNotAvailable,
};
use crate::routes::auth::RegisterModel;
use crate::services::emails::{Email, EmailTemplate};
use crate::services::env::EnvConfig;
use crate::utils::app_state;
use crate::utils::cookies::{ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE, create_cookie};
use crate::utils::error::Error;
use crate::utils::error::Error::HashPasswordFailed;
use crate::utils::jwt::encode_jwt;
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

        let user_info =
            UserInformation::new(&app_state.database, password_hash, &credentials).await?;

        let confirmation_code =
            email_confirmation::Model::create_with_userinfo(&app_state.database, user_info).await?;

        let confirmation_link = Self::create_email_confirmation_link(&confirmation_code)?;

        let sender_email = format!("auth@{}", &EnvConfig::get().domain);

        Email {
            sender: (Some("Autoryzacja Hack4Krak".to_string()), sender_email),
            recipients: vec![credentials.email],
            subject: "Potwierdzenie rejestracji".to_string(),
            template: EmailTemplate::EmailConfirmation,
            placeholders: Some(vec![
                ("user".to_string(), credentials.name),
                ("link".to_string(), confirmation_link.to_string()),
            ]),
        }
        .send(app_state)
        .await?;

        Ok(HttpResponse::Ok().body("User successfully registered."))
    }

    pub fn assert_password_is_valid(
        user: &users::Model,
        password_to_verify: &str,
    ) -> Result<(), Error> {
        let password = user
            .password
            .clone()
            .ok_or(Error::Auth(PasswordAuthNotAvailable))?;

        let parsed_hash = PasswordHash::new(&password).map_err(Error::HashPasswordFailed)?;

        let is_verified = Argon2::default()
            .verify_password(password_to_verify.as_bytes(), &parsed_hash)
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
        confirmation_code: String,
    ) -> Result<(), Error> {
        let user_info = email_confirmation::Model::get_user_info(
            &app_state.database,
            confirmation_code.clone(),
        )
        .await?;

        users::Model::create_from_user_info(&app_state.database, user_info).await?;

        email_confirmation::Model::remove_expired_and_confirmed(
            &app_state.database,
            Some(confirmation_code),
        )
        .await?;

        Ok(())
    }

    fn create_email_confirmation_link(confirmation_code: &str) -> Result<String, Error> {
        let mut url = EnvConfig::get().backend_url.clone();
        url = url.join("/auth/confirm/")?;
        url = url.join(confirmation_code)?;

        Ok(url.to_string())
    }

    pub fn hash_password(password: String) -> Result<String, Error> {
        let salt = SaltString::generate(&mut OsRng);

        Ok(Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(HashPasswordFailed)?
            .to_string())
    }

    pub async fn update_user(
        app_state: &app_state::AppState,
        user: users::Model,
        model: UpdateUserModel,
    ) -> Result<(), Error> {
        Self::assert_password_is_valid(&user, &model.old_password)?;

        if let Some(password) = model.new_password {
            let password_hash = Self::hash_password(password)?;
            users::Model::update(
                &app_state.database,
                user,
                model.username,
                Some(password_hash),
            )
            .await?;

            return Ok(());
        }

        users::Model::update(&app_state.database, user, model.username, None).await?;

        Ok(())
    }
}
