use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::cookies::{create_cookie, reset_cookie};

#[test]
fn reset_cookie_contains_name() {
    EnvConfig::load_test_config();

    let cookie = reset_cookie("session");
    assert!(cookie.contains("session=removed"));
}

#[test]
fn reset_cookie_has_past_expiry() {
    EnvConfig::load_test_config();

    let cookie = reset_cookie("token");
    assert!(cookie.contains("Expires="));
}

#[test]
fn create_cookie_contains_name_and_value() {
    EnvConfig::load_test_config();

    let cookie = create_cookie("access_token", "abc123", None);
    assert!(cookie.contains("access_token=abc123"));
}

#[test]
fn create_cookie_is_http_only() {
    EnvConfig::load_test_config();

    let cookie = create_cookie("token", "value", None);
    assert!(cookie.contains("HttpOnly"));
}

#[test]
fn create_cookie_is_secure() {
    EnvConfig::load_test_config();

    let cookie = create_cookie("token", "value", None);
    assert!(cookie.contains("Secure"));
}

#[test]
fn create_cookie_is_same_site_strict() {
    EnvConfig::load_test_config();

    let cookie = create_cookie("token", "value", None);
    assert!(cookie.contains("SameSite=Strict"));
}

#[test]
fn create_cookie_with_max_age() {
    EnvConfig::load_test_config();

    let max_age = actix_web::cookie::time::Duration::hours(1);
    let cookie = create_cookie("token", "value", Some(max_age));
    assert!(cookie.contains("Max-Age="));
}
