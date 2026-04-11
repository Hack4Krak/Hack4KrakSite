use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;

#[actix_web::test]
async fn get_personal_information_none() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/account/get_personal_information")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response: serde_json::Value = test::call_and_read_body_json(&app, request).await;
    assert!(response.is_null());
}

#[actix_web::test]
async fn get_personal_information_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/account/get_personal_information")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}
