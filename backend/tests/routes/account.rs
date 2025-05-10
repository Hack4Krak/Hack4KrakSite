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
async fn submit_personal_info() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/account/submit_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
          "birth_year": 2000,
          "first_name": "Antonio",
          "is_vegetarian": true,
          "location": "Włochy",
          "marketing_consent": true,
          "organization": "Hack4Krak",
          "referral_source": ["Linkedin"]
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
        r#"{"username":"test_user","email":"example@gmail.com","has_personal_information":true}"#
    );

    let request = test::TestRequest::get()
        .uri("/account/get_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn submit_personal_info_invalid_referral_source() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/account/submit_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
          "birth_year": 2000,
          "first_name": "Antonio",
          "is_vegetarian": true,
          "location": "Włochy",
          "marketing_consent": true,
          "organization": "Hack4Krak",
          "referral_source": ["Linkedin", "Invalid"]
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn submit_personal_info_invalid_birth_year() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/account/submit_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
          "birth_year": 3000,
          "first_name": "Antonio",
          "is_vegetarian": true,
          "location": "Włochy",
          "marketing_consent": true,
          "organization": "Hack4Krak",
          "referral_source": ["Linkedin"]
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}
