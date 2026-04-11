use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use hack4krak_backend::services::env::EnvConfig;

#[actix_web::test]
async fn metrics_without_auth_unauthorized() {
    let test_db = TestDatabase::new().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get().uri("/metrics").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn metrics_with_wrong_token() {
    let test_db = TestDatabase::new().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/metrics")
        .insert_header(("Authorization", "Bearer wrong_token"))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn metrics_with_valid_token() {
    EnvConfig::load_test_config();
    let token = &EnvConfig::get().metrics_access_token;

    let test_db = TestDatabase::new().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/metrics")
        .insert_header(("Authorization", format!("Bearer {token}")))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}
