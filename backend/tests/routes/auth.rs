use crate::test_utils::TestApp;

use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use actix_web::test::read_body_json;
use chrono::Utc;
use hack4krak_backend::entities::email_verification_request;
use sea_orm::{EntityTrait, Set};
use serde_json::json;
use uuid::Uuid;

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
    let test_database = TestDatabase::new().await;

    let confirmation_code = Uuid::new_v4();
    let email_confirmation = email_verification_request::ActiveModel {
        id: Set(confirmation_code),
        email: Set("".to_string()),
        action_type: Set("confirm_email_address".to_string()),
        additional_data: Set(Some(json![{
            "user_information": {
                "name": "test_user",
                "email": "example@gmail.com",
                "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$nTzWdmrtGEOnwCocrg76xg$yv16FfDT5+meKwPmSiV+MF9kP8Man6bXZs+BloFTKIk".to_string(),
            }
        }])),
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
}

#[actix_web::test]
async fn email_confirmation_expired() {
    let test_database = TestDatabase::new().await;

    // todo: move it to proper place
    let confirmation_code = Uuid::new_v4();
    let email_confirmation = email_verification_request::ActiveModel {
        id: Set(confirmation_code),
        email: Set("".to_string()),
        action_type: Set("confirm_email_address".to_string()),
        additional_data: Set(Some(json![{
            "name": "test_user",
            "email": "example@gmail.com",
            "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$nTzWdmrtGEOnwCocrg76xg$yv16FfDT5+meKwPmSiV+MF9kP8Man6bXZs+BloFTKIk".to_string(),
        }])),
        expiration_time: Set(Some(Utc::now().naive_utc())),
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

    let email_body = &smtp_client.get_emails().await.items[0].content.body;
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
async fn logout_returns_reset_cookies() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post().uri("/auth/logout").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let set_cookies: Vec<_> = response
        .headers()
        .get_all("Set-Cookie")
        .map(|v| v.to_str().unwrap().to_string())
        .collect();
    assert!(set_cookies.iter().any(|c| c.contains("access_token")));
    assert!(set_cookies.iter().any(|c| c.contains("refresh_token")));
}

#[actix_web::test]
async fn login_invalid_credentials() {
    let test_database = TestDatabase::new().await;
    test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(json!({
            "email": "example@gmail.com",
            "password": "wrong_password"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn login_nonexistent_user() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(json!({
            "email": "nobody@example.com",
            "password": "password123"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn auth_status_with_cookie() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::get()
        .uri("/auth/status")
        .insert_header(("Cookie", "refresh_token=some_value"))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn auth_status_without_cookie() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::get().uri("/auth/status").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn refresh_without_cookie() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post().uri("/auth/refresh").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn refresh_with_invalid_cookie() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/auth/refresh")
        .insert_header(("Cookie", "refresh_token=invalid_jwt_token"))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn login_success() {
    let test_database = TestDatabase::new().await;
    test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(json!({
            "email": "example@gmail.com",
            "password": "Dziengiel"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let set_cookies: Vec<_> = response
        .headers()
        .get_all("Set-Cookie")
        .map(|v| v.to_str().unwrap().to_string())
        .collect();
    assert!(set_cookies.iter().any(|c| c.contains("access_token")));
    assert!(set_cookies.iter().any(|c| c.contains("refresh_token")));
}

#[actix_web::test]
async fn login_missing_fields() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(json!({ "email": "example@gmail.com" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn register_password_too_short() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "test@example.com",
            "name": "test_user",
            "password": "short"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn register_username_too_short() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({
            "email": "test@example.com",
            "name": "ab",
            "password": "password123"
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn email_confirmation_not_found() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri(&format!("/auth/confirm/{}", uuid::Uuid::new_v4()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error() || response.status().is_server_error());
}

#[actix_web::test]
async fn login_then_access_account() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/account/")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response: serde_json::Value = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response["username"], "test_user");
    assert_eq!(response["email"], "example@gmail.com");
}

#[actix_web::test]
async fn register_missing_fields() {
    let app = TestApp::default().build_app().await;

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(json!({ "email": "test@example.com" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}
