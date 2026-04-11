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
        r#"{"username":"Salieri","email":"example@gmail.com","has_personal_information":false}"#
    );

    let request = test::TestRequest::patch()
        .uri("/account/update/password")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
            "old_password": "Dziengiel",
            "new_password": "Dziengiel2"
        }))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

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

#[actix_web::test]
async fn update_username_too_short() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update")
        .insert_header(TestAuthHeader::new(user))
        .set_json(serde_json::json!({ "username": "ab" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn update_username_too_long() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update")
        .insert_header(TestAuthHeader::new(user))
        .set_json(serde_json::json!({ "username": "a".repeat(33) }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn update_username_special_chars() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update")
        .insert_header(TestAuthHeader::new(user))
        .set_json(serde_json::json!({ "username": "user@name!" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn update_username_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update")
        .set_json(serde_json::json!({ "username": "NewName" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn change_password_wrong_old_password() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update/password")
        .insert_header(TestAuthHeader::new(user))
        .set_json(serde_json::json!({
            "old_password": "WrongPassword123",
            "new_password": "NewPassword123"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn change_password_new_too_short() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update/password")
        .insert_header(TestAuthHeader::new(user))
        .set_json(serde_json::json!({
            "old_password": "Dziengiel",
            "new_password": "short"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn change_password_old_too_short() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update/password")
        .insert_header(TestAuthHeader::new(user))
        .set_json(serde_json::json!({
            "old_password": "short",
            "new_password": "ValidPassword123"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn change_password_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update/password")
        .set_json(serde_json::json!({
            "old_password": "Dziengiel",
            "new_password": "NewPassword123"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}
