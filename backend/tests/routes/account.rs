use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;

#[actix_web::test]
async fn account_delete() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/account/delete")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/account/")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn account_update() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
            "username": "Salieri",
            "old_password": "Dziengiel",
            "new_password": "Dziengiel2"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/account/")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_and_read_body(&app, request).await;
    assert_eq!(
        response,
        r#"{"username":"Salieri","email":"example@gmail.com"}"#
    );

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(serde_json::json!({
            "email": "example@gmail.com",
            "password": "Dziengiel2"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}
