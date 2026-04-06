use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use crate::test_utils::task_manager::create_default_test_task_manager;
use actix_web::test;
use actix_web::test::read_body_json;
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::users;
use hack4krak_backend::services::verification::VerifiedUserInfo;
use serde::{Deserialize, Serialize};
use serde_json::json;
use uuid::Uuid;

#[derive(Serialize, Deserialize)]
struct ApplyTagResponse {
    pub user_info: VerifiedUserInfo,
}

#[actix_web::test]
async fn identify_user_success() {
    let test_database = TestDatabase::new().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let user = test_database.with_default_user().await;
    let verification_id = user.verification_id;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/admin/verification/identify")
        .set_json(json!({
            "verification_id": verification_id
        }))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let body: VerifiedUserInfo = read_body_json(response).await;
    assert_eq!(body.user_id, user.id);
    assert_eq!(body.username, user.username);
    assert_eq!(body.email, user.email);
}

#[actix_web::test]
async fn identify_user_invalid_uuid() {
    let test_database = TestDatabase::new().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let invalid_uuid = Uuid::new_v4();

    let request = test::TestRequest::post()
        .uri("/admin/verification/identify")
        .set_json(json!({
            "verification_id": invalid_uuid
        }))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn apply_tag_success() {
    let test_database = TestDatabase::new().await;
    let task_manager = create_default_test_task_manager().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let user = test_database.with_default_user().await;
    let verification_id = user.verification_id;

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/admin/verification/apply-tag")
        .set_json(json!({
            "verification_id": verification_id,
            "tag_id": "sponsor"
        }))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let body: ApplyTagResponse = read_body_json(response).await;
    assert_eq!(body.user_info.user_id, user.id);
}

#[actix_web::test]
async fn apply_tag_invalid_tag_id() {
    let test_database = TestDatabase::new().await;
    let task_manager = create_default_test_task_manager().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let user = test_database.with_default_user().await;
    let verification_id = user.verification_id;

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/admin/verification/apply-tag")
        .set_json(json!({
            "verification_id": verification_id,
            "tag_id": "nonexistent_tag"
        }))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn apply_tag_invalid_uuid() {
    let test_database = TestDatabase::new().await;
    let task_manager = create_default_test_task_manager().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let invalid_uuid = Uuid::new_v4();

    let request = test::TestRequest::post()
        .uri("/admin/verification/apply-tag")
        .set_json(json!({
            "verification_id": invalid_uuid,
            "tag_id": "sponsor"
        }))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn confirm_tag_invalid_tag_id() {
    let test_database = TestDatabase::new().await;
    let task_manager = create_default_test_task_manager().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/admin/verification/confirm-tag")
        .set_json(json!({
            "user_id": user.id,
            "tag_id": "nonexistent_tag"
        }))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn reset_uuid_success() {
    use crate::test_utils::mail::SmtpTestClient;

    let test_database = TestDatabase::new().await;
    let smtp_client = SmtpTestClient::new().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .with_smtp_client(smtp_client.smtp_client.clone())
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri(&format!("/admin/verification/reset/{}", user.id))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    // Verify email was sent
    let emails = smtp_client.get_emails().await;
    assert_eq!(emails.items.len(), 1);
    assert_eq!(emails.items[0].raw.to[0].as_str(), user.email);
}

#[actix_web::test]
async fn reset_uuid_user_not_found() {
    let test_database = TestDatabase::new().await;
    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let nonexistent_user_id = Uuid::new_v4();

    let request = test::TestRequest::post()
        .uri(&format!(
            "/admin/verification/reset/{}",
            nonexistent_user_id
        ))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}
