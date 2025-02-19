use crate::services::env::EnvConfig;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::cookie::{Cookie, SameSite};

pub const ACCESS_TOKEN_COOKIE: &str = "access_token";
pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

pub fn reset_cookie(name: &str) -> String {
    let cookie = Cookie::build(name, "removed")
        .expires(OffsetDateTime::UNIX_EPOCH)
        .path("/")
        .domain(EnvConfig::get().cookies_domain.clone())
        .secure(true)
        .finish()
        .to_string();

    if EnvConfig::get().relaxed_security_mode {
        return format!("{}; Partitioned", cookie);
    }

    cookie
}

pub fn create_cookie(name: &str, value: &str, max_age: Option<Duration>) -> String {
    let mut cookie = Cookie::build(name, value)
        .path("/")
        .http_only(true)
        .domain(EnvConfig::get().cookies_domain.clone())
        // Most browsers don't verify it for localhost
        .secure(true);

    if let Some(max_age) = max_age {
        cookie = cookie.max_age(max_age);
    }

    if EnvConfig::get().relaxed_security_mode {
        // Actix doesn't yet support Partitioned cookies
        // https://github.com/actix/actix-web/pull/3336
        format!("{}; Partitioned", cookie.same_site(SameSite::None).finish())
    } else {
        cookie.same_site(SameSite::Strict).finish().to_string()
    }
}
