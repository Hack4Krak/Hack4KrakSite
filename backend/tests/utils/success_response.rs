use hack4krak_backend::utils::success_response::SuccessResponse;

#[test]
fn default_success_response_is_200() {
    let response = SuccessResponse::default().http_response();
    assert_eq!(response.status(), 200);
}

#[test]
fn default_success_response_has_ok_status() {
    let response = SuccessResponse::default().http_response();
    assert!(response.status().is_success());
}
