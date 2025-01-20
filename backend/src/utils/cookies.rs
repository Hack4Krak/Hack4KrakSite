use actix_web::cookie::time::Duration;
use actix_web::cookie::{Cookie, SameSite};

pub const ACCESS_TOKEN_COOKIE: &str = "access_token";
pub const REFRESH_TOKEN_COOKIE: &str = "refresh_token";

pub fn create_cookie(name: &str, value: &str, max_age: Option<Duration>) -> String {
    let mut cookie = Cookie::build(name, value)
        .path("/")
        .http_only(true)
        .same_site(SameSite::Strict)
        .domain("localhost")
        // Most browsers don't verify it for localhost
        .secure(true);

    if let Some(max_age) = max_age {
        cookie = cookie.max_age(max_age);
    }

    cookie.finish().to_string()
}
