#![cfg(feature = "full-test-suite")]
use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use crate::test_utils::mail::SmtpTestClient;
use actix_web::test;
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::users;
use serde_json::json;

#[actix_web::test]
async fn external_invitations_flow() {
    let test_database = TestDatabase::new().await;
    let smtp_client = SmtpTestClient::new().await;
    let user = test_database.with_default_user().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .with_smtp_client(smtp_client.smtp_client.clone())
        .build_app()
        .await;

    // Register organization as admin
    let request = test::TestRequest::post()
        .uri("/admin/email/send_external_registration_form")
        .set_json(json!({
            "organization": "School for Silly Crabs".to_string(),
            "email_address": "crab@norbiros.dev".to_string(),
        }))
        .insert_header(TestAuthHeader::new(admin.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 200);

    let confirmation_code = smtp_client.find_uuid_in_first_email().await;

    let request = test::TestRequest::post()
        .uri(&format!(
            "/teams/external_invitations/create/{}",
            confirmation_code
        ))
        .set_json(json!({
            // Testing maximum, average, and minimum team sizes
            "teams": [["Duże Kotki", 5], ["Kotki", 5], ["Kraby", 1]]
        }))
        .insert_header(TestAuthHeader::new(admin.clone()))
        .to_request();
    let response: Vec<Vec<String>> = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response.len(), 3);

    // Normal user joins
    let request = test::TestRequest::post()
        .uri("/teams/external_invitations/join")
        .set_json(json!({
            "code": response[0][0].clone()
        }))
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 200);

    // Admin displays status
    let request = test::TestRequest::get()
        .uri(&format!(
            "/teams/external_invitations/info/{}",
            confirmation_code
        ))
        .insert_header(TestAuthHeader::new(admin.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 200);
}
