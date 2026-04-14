use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::common_responses::{
    create_redirect_response, create_temporary_redirect_response,
};
use hack4krak_backend::utils::error::Error;
use url::Url;

#[test]
fn create_redirect_response_sets_refresh_header() {
    EnvConfig::load_test_config();

    let url = Url::parse("https://example.com/callback").unwrap();
    let mut response = create_redirect_response(url.clone()).unwrap();
    let response = response.finish();

    assert_eq!(response.status(), 200);

    let refresh_header = response.headers().get("Refresh").unwrap();
    assert!(
        refresh_header
            .to_str()
            .unwrap()
            .contains("https://example.com/callback")
    );
}

#[test]
fn create_redirect_response_contains_url() {
    EnvConfig::load_test_config();

    let url = Url::parse("https://example.com/path?key=value").unwrap();
    let mut response = create_redirect_response(url).unwrap();
    let response = response.finish();

    let header = response.headers().get("Refresh").unwrap().to_str().unwrap();
    assert!(header.starts_with("0; "));
    assert!(header.contains("key=value"));
}

#[test]
fn create_temporary_redirect_response_returns_307() {
    EnvConfig::load_test_config();

    let url = Url::parse("https://example.com/error").unwrap();
    let error = Error::Unauthorized;
    let mut response = create_temporary_redirect_response(url, error).unwrap();
    let response = response.finish();

    assert_eq!(response.status(), 307);
}

#[test]
fn create_temporary_redirect_response_sets_location_header() {
    EnvConfig::load_test_config();

    let url = Url::parse("https://example.com/error").unwrap();
    let error = Error::Unauthorized;
    let mut response = create_temporary_redirect_response(url, error).unwrap();
    let response = response.finish();

    let location = response
        .headers()
        .get("Location")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(location.starts_with("https://example.com/error"));
}

#[test]
fn create_temporary_redirect_response_includes_error_as_query_param() {
    EnvConfig::load_test_config();

    let url = Url::parse("https://example.com/error").unwrap();
    let error = Error::Unauthorized;
    let mut response = create_temporary_redirect_response(url, error).unwrap();
    let response = response.finish();

    let location = response
        .headers()
        .get("Location")
        .unwrap()
        .to_str()
        .unwrap();
    assert!(location.contains("error="));
}
