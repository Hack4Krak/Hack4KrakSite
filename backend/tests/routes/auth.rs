use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use actix_web::test::read_body_json;
use serde_json::json;

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn register() {
    use crate::test_utils::mail::SmtpTestClient;

    let test_database = TestDatabase::new().await;
    let smtp_client = SmtpTestClient::new().await;
    let app = TestApp::default()
        .with_database(test_database)
        .with_smtp_client(smtp_client.smtp_client.clone())
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "test@example.com",
            "name": "test_user",
            "first_name": "Test",
            "password": "password123"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let first_email = &smtp_client.get_emails().await.items[0];
    assert_eq!(first_email.raw.to[0].as_str(), "test@example.com");
}

#[actix_web::test]
async fn register_invalid_email() {
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
    assert!(response.status().is_client_error());
    // Verify if the body is proper JSON
    let _: serde_json::Value = read_body_json(response).await;
}

#[actix_web::test]
async fn register_invalid_username() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "email@example.com",
            "name": "test ﷽ user",
            "first_name": "Test",
            "password": "password123"
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn auth_flow() {
    use crate::test_utils::mail::SmtpTestClient;
    use actix_http::header;
    use actix_web::cookie::Cookie;

    let test_database = TestDatabase::new().await;
    let smtp_client = SmtpTestClient::new().await;
    let app = TestApp::default()
        .with_database(test_database)
        .with_smtp_client(smtp_client.smtp_client.clone())
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "test@example.com",
            "name": "test_user",
            "first_name": "Test",
            "password": "password123"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let confirmation_code = smtp_client.find_uuid_in_first_email().await;
    let request = test::TestRequest::get()
        .uri(&format!("/auth/confirm/{confirmation_code}"))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(json!({
            "email": "test@example.com",
            "password": "password123"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let access_token = response
        .headers()
        .get_all(header::SET_COOKIE)
        .map(|set_cookie| Cookie::parse(set_cookie.to_str().unwrap()).unwrap())
        .find(|cookie| cookie.name() == "access_token")
        .unwrap();

    let user_request = test::TestRequest::get()
        .uri("/account/")
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.value()),
        ))
        .to_request();
    let user_response = test::call_service(&app, user_request).await;
    assert!(user_response.status().is_success());
}

#[actix_web::test]
async fn email_confirmation_success() {
    use hack4krak_backend::entities::email_verification_request::UpdatableModel;
    use hack4krak_backend::entities::{email_verification_request, users};
    use hack4krak_backend::services::authentication::AuthenticationService;
    use hack4krak_backend::utils::app_state::AppState;
    use sea_orm::EntityTrait;

    let test_database = TestDatabase::new().await;
    let email_confirmation = test_database
        .with_email_verification_request(UpdatableModel::default())
        .await;
    let confirmation_code = email_confirmation.id;

    let app_state = AppState::with_database(test_database.database);

    let result = AuthenticationService::confirm_email(&app_state, confirmation_code).await;

    assert!(result.is_ok());

    let user = users::Model::find_by_email(&app_state.database, "example@gmail.com")
        .await
        .unwrap();
    assert!(user.is_some());

    let req = email_verification_request::Entity::find_by_id(confirmation_code)
        .one(&app_state.database)
        .await
        .unwrap();
    assert!(req.is_none());
}

#[actix_web::test]
async fn email_confirmation_expired() {
    use chrono::Utc;
    use hack4krak_backend::entities::email_verification_request::UpdatableModel;

    let test_database = TestDatabase::new().await;

    let email_confirmation = test_database
        .with_email_verification_request(UpdatableModel {
            expiration_time: Some(Some(Utc::now().naive_utc() - chrono::Duration::minutes(1))),
            ..Default::default()
        })
        .await;
    let confirmation_code = email_confirmation.id;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let path = format!("/auth/confirm/{confirmation_code}");
    let request = test::TestRequest::get().uri(&path).to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 307);
}

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn reset_password_flow() {
    use crate::test_utils::mail::SmtpTestClient;

    const UUID_REGEX: &str =
        r"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}";
    let regex = regex::Regex::new(UUID_REGEX).unwrap();

    let test_database = TestDatabase::new().await;
    test_database.with_default_user().await;

    let smtp_client = SmtpTestClient::new().await;
    let app = TestApp::default()
        .with_database(test_database)
        .with_smtp_client(smtp_client.smtp_client.clone())
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/auth/request_reset_password")
        .set_json(json!({
            "email": "example@gmail.com"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 200);

    let email_body = &smtp_client.wait_for_emails(1).await.items[0].content.body;
    let reset_code = regex.find(email_body).unwrap().as_str();

    let request = test::TestRequest::patch()
        .uri("/auth/reset_password")
        .set_json(json!({
            "code": reset_code.to_string(),
            "new_password": "meow123123".to_string()
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 200);
}

#[actix_web::test]
async fn email_confirmation_with_callback() {
    use chrono::Utc;
    use hack4krak_backend::entities::email_verification_request;
    use sea_orm::{EntityTrait, Set};
    use uuid::Uuid;

    let test_database = TestDatabase::new().await;

    let confirmation_code = Uuid::new_v4();
    let email_confirmation = email_verification_request::ActiveModel {
        id: Set(confirmation_code),
        email: Set("".to_string()),
        action_type: Set("confirm_email_address".to_string()),
        additional_data: Set(Some(json!({
            "user_information": {
                "name": "test_user",
                "email": "example@gmail.com",
                "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$nTzWdmrtGEOnwCocrg76xg$yv16FfDT5+meKwPmSiV+MF9kP8Man6bXZs+BloFTKIk"
            },
            "callback": "/tasks"
        }))),
        expiration_time: Set(Some(Utc::now().naive_utc() + chrono::Duration::minutes(30))),
        created_at: Set(Utc::now().naive_utc()),
    };
    email_verification_request::Entity::insert(email_confirmation)
        .exec(&test_database.database)
        .await
        .unwrap();

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let path = format!("/auth/confirm/{confirmation_code}");
    let request = test::TestRequest::get().uri(&path).to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let refresh_header = response
        .headers()
        .get("Refresh")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        refresh_header.contains("callback=%2Ftasks"),
        "Redirect should contain callback parameter, got: {refresh_header}"
    );
}

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn register_with_callback_persists_to_confirmation() {
    use crate::test_utils::mail::SmtpTestClient;

    let test_database = TestDatabase::new().await;
    let smtp_client = SmtpTestClient::new().await;
    let app = TestApp::default()
        .with_database(test_database)
        .with_smtp_client(smtp_client.smtp_client.clone())
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "test@example.com",
            "name": "test_user",
            "first_name": "Test",
            "password": "password123",
            "callback": "/panel/tasks"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let confirmation_code = smtp_client.find_uuid_in_first_email().await;
    let request = test::TestRequest::get()
        .uri(&format!("/auth/confirm/{confirmation_code}"))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let refresh_header = response
        .headers()
        .get("Refresh")
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();
    assert!(
        refresh_header.contains("callback=%2Fpanel%2Ftasks"),
        "Callback should persist from registration to confirmation redirect, got: {refresh_header}"
    );
}

#[actix_web::test]
async fn register_with_invalid_callback_rejected() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "test@example.com",
            "name": "test_user",
            "first_name": "Test",
            "password": "password123",
            "callback": "https://evil.com"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(
        response.status().is_client_error(),
        "Callback with absolute URL should be rejected"
    );
}
