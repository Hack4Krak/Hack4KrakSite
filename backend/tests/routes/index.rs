use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use serde_json::Value;

#[actix_web::test]
async fn index_response() {
    let test_db = TestDatabase::new().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get().uri("/").to_request();
    let response: Value = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response["name"], "Hack4Krak");
    assert!(response.get("version").is_some());
    assert!(response.get("about").is_some());
}

#[actix_web::test]
async fn route_not_found() {
    let test_db = TestDatabase::new().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get().uri("/nonexistent").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}
