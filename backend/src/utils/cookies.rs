use crate::services::env::EnvConfig;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::cookie::{Cookie, SameSite};

pub const ACCESS_TOKEN_COOKIE: &str = "access_token";
pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

pub fn reset_cookie(name: &str) -> String {
    Cookie::build(name, "removed")
        .expires(OffsetDateTime::UNIX_EPOCH)
        .path("/")
        .domain(EnvConfig::get().cookies_domain.clone())
        .secure(true)
        .finish()
        .to_string()
}

pub fn create_cookie(name: &str, value: &str, max_age: Option<Duration>) -> String {
    let mut cookie = Cookie::build(name, value)
        .path("/")
        .http_only(true)
        .domain(EnvConfig::get().cookies_domain.clone())
        .same_site(SameSite::Strict)
        // Most browsers don't verify it for localhost
        .secure(true);

    if let Some(max_age) = max_age {
        cookie = cookie.max_age(max_age);
    }

    cookie.finish().to_string()
}
