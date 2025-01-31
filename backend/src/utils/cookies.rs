use crate::utils::env::Config;
use actix_web::cookie::time::{Duration, OffsetDateTime};
use actix_web::cookie::{Cookie, SameSite};

pub const ACCESS_TOKEN_COOKIE: &str = "access_token";
pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

pub fn reset_cookie(name: &str) -> Cookie {
    Cookie::build(name, "removed")
        .expires(OffsetDateTime::UNIX_EPOCH)
        .path("/")
        .domain(Config::get().cookies_domain.clone())
        .finish()
}

pub fn create_cookie(name: &str, value: &str, max_age: Option<Duration>) -> String {
    let mut cookie = Cookie::build(name, value)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .domain(Config::get().cookies_domain.clone())
        // Most browsers don't verify it for localhost
        .secure(true);

    if let Some(max_age) = max_age {
        cookie = cookie.max_age(max_age);
    }

    cookie.finish().to_string()
}
