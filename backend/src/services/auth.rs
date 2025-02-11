use crate::entities::users;
use crate::routes::auth::AuthError::{
    InvalidCredentials, InvalidEmailAddress, PasswordAuthNotAvailable,
};
use crate::routes::auth::RegisterModel;
use crate::utils::cookies::{create_cookie, ACCESS_TOKEN_COOKIE, REFRESH_TOKEN_COOKIE};
use crate::utils::error::Error;
use crate::utils::error::Error::HashPasswordFailed;
use crate::utils::jwt::encode_jwt;
use actix_web::{HttpResponse, HttpResponseBuilder};
use argon2::password_hash::rand_core::OsRng;
use argon2::password_hash::SaltString;
use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use chrono::Duration;
use regex::Regex;
use sea_orm::DatabaseConnection;
use uuid::Uuid;

const EMAIL_REGEX: &str =
    r"^([a-z0-9_+]([a-z0-9_+.]*[a-z0-9_+])?)@([a-z0-9]+([\-\.]{1}[a-z0-9]+)*\.[a-z]{2,6})$";

pub struct AuthService;

impl AuthService {
    pub async fn register_with_password(
        database: &DatabaseConnection,
        credentials: RegisterModel,
    ) -> Result<HttpResponse, Error> {
        let regex = Regex::new(EMAIL_REGEX).unwrap();
        if !regex.is_match(&credentials.email) {
            return Err(Error::Auth(InvalidEmailAddress));
        }

        let salt = SaltString::generate(&mut OsRng);
        let password_hash = Argon2::default()
            .hash_password(credentials.password.as_bytes(), &salt)
            .map_err(HashPasswordFailed)?
            .to_string();

        let uuid = Uuid::new_v4();
        users::Model::create_with_password(database, uuid, password_hash, &credentials).await?;

        Self::response_with_cookies(uuid, credentials.email)
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
}
