use actix_web::test::TestRequest;
use hack4krak_backend::utils::bearer::verify_bearer_token;

#[test]
fn valid_bearer_token() {
    let req = TestRequest::default()
        .insert_header(("Authorization", "Bearer secret-token"))
        .to_http_request();

    let result = verify_bearer_token(&req, "secret-token");
    assert!(result.is_ok());
}

#[test]
fn missing_authorization_header() {
    let req = TestRequest::default().to_http_request();

    let result = verify_bearer_token(&req, "secret-token");
    assert!(result.is_err());
}

#[test]
fn wrong_token() {
    let req = TestRequest::default()
        .insert_header(("Authorization", "Bearer wrong-token"))
        .to_http_request();

    let result = verify_bearer_token(&req, "secret-token");
    assert!(result.is_err());
}

#[test]
fn missing_bearer_prefix() {
    let req = TestRequest::default()
        .insert_header(("Authorization", "secret-token"))
        .to_http_request();

    let result = verify_bearer_token(&req, "secret-token");
    assert!(result.is_err());
}

#[test]
fn empty_token() {
    let req = TestRequest::default()
        .insert_header(("Authorization", "Bearer "))
        .to_http_request();

    let result = verify_bearer_token(&req, "secret-token");
    assert!(result.is_err());
}
