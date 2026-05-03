use crate::test_utils::TestApp;
use actix_http::body::MessageBody;
use actix_web::test;
use actix_web::test::read_body_json;
use serde_json::json;

async fn assert_correct_error_format<B: MessageBody + 'static>(
    response: actix_web::dev::ServiceResponse<B>,
) {
    assert_eq!(response.status(), 400);
    let body: serde_json::Value = read_body_json(response).await;
    assert!(
        body.get("message").is_some(),
        "Response should contain 'message' field"
    );
    assert!(
        body.get("code").is_some(),
        "Response should contain 'code' field"
    );
    assert!(
        body.get("error").is_some(),
        "Response should contain 'error' field"
    );
}

#[actix_web::test]
async fn correct_uuid_error_format() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::get()
        .uri("/auth/confirm/not-a-valid-uuid")
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_correct_error_format(response).await;
}

#[actix_web::test]
async fn correct_json_error_format() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .insert_header(("Content-Type", "application/json"))
        .set_payload(r#"{"email": "test@example.com", "name": invalid_json}"#)
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_correct_error_format(response).await;
}

#[actix_web::test]
async fn correct_validation_error_format() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "this_!isn'taemaill",
            "name": "test_user",
            "first_name": "Test",
            "password": "password123"
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_correct_error_format(response).await;
}
