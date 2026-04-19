use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use crate::test_utils::task_manager::create_default_test_task_manager;
use actix_web::test;
use actix_web::test::read_body_json;
use hack4krak_backend::entities::sea_orm_active_enums::{TeamStatus, UserRoles};
use hack4krak_backend::entities::{teams, users};
use hack4krak_backend::routes::teams::MyTeamWithMembers;
use hack4krak_backend::services::identification::IdentifiedUserInfo;
use sea_orm::EntityTrait;
use serde_json::json;
use uuid::Uuid;

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
    let identification_code = user.identification_code;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri(&format!(
            "/admin/identification/identify/{identification_code}"
        ))
        .insert_header(TestAuthHeader::new(admin.id.clone(), admin.email.clone()))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let body: IdentifiedUserInfo = read_body_json(response).await;
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
        .uri("/admin/identification/identify")
        .set_json(json!({
            "identification_code": invalid_uuid
        }))
        .insert_header(TestAuthHeader::new(admin.id.clone(), admin.email.clone()))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
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

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/admin/identification/apply-tag")
        .set_json(json!({
            "identification_code": user.identification_code,
            "tag_id": "nonexistent_tag"
        }))
        .insert_header(TestAuthHeader::new(admin.id.clone(), admin.email.clone()))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn apply_tag_first_verified_user_marks_team_as_confirmed() {
    let test_database = TestDatabase::new().await;
    let task_manager = create_default_test_task_manager().await;

    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let team = test_database.with_default_team().await;

    let first_user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let _second_user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let initial_team = teams::Entity::find_by_id(team.id)
        .one(&test_database.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(initial_team.status, TeamStatus::Absent);

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri(
            format!(
                "/admin/identification/apply-tag/{}",
                first_user.identification_code
            )
            .as_str(),
        )
        .set_json(json!({
            "tag_id": "present-on-event"
        }))
        .insert_header(TestAuthHeader::new(admin.id, admin.email))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(TestAuthHeader::new(first_user.id, first_user.email))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let body: MyTeamWithMembers = read_body_json(response).await;

    assert_eq!(body.status, TeamStatus::Confirmed);
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
        .uri(&format!("/admin/identification/reset/{}", user.id))
        .insert_header(TestAuthHeader::new(admin.id.clone(), admin.email.clone()))
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
            "/admin/identification/reset/{}",
            nonexistent_user_id
        ))
        .insert_header(TestAuthHeader::new(admin.id.clone(), admin.email.clone()))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn full_identity_identification_flow() {
    use crate::test_utils::mail::SmtpTestClient;
    use hack4krak_backend::services::identification::UserIdentificationInfo;

    let test_database = TestDatabase::new().await;
    let mock_smtp_client = SmtpTestClient::new().await;
    let task_manager = create_default_test_task_manager().await;

    let admin = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            email: Some("admin@niepodam.pl".to_string()),
            username: Some("admin".to_string()),
            ..Default::default()
        })
        .await;

    let confirmation_code = test_database
        .with_email_verification_request(Default::default())
        .await
        .id;

    let app = TestApp::default()
        .with_database(test_database)
        .with_smtp_client(mock_smtp_client.smtp_client.clone())
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri(&format!("/auth/confirm/{confirmation_code}"))
        .to_request();

    test::call_service(&app, request).await;

    let identification_code = mock_smtp_client.find_uuid_in_first_email().await;

    let request = test::TestRequest::get()
        .uri(format!("/admin/identification/identify/{identification_code}").as_str())
        .insert_header(TestAuthHeader::new(admin.id.clone(), admin.email.clone()))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
    let response_body: IdentifiedUserInfo = read_body_json(response).await;

    let request = test::TestRequest::post()
        .uri(format!("/admin/identification/apply-tag/{identification_code}").as_str())
        .set_json(json!({
            "tag_id": "present-on-event"
        }))
        .insert_header(TestAuthHeader::new(admin.id.clone(), admin.email.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success(), "Apply tag should succeed");

    let request = test::TestRequest::get()
        .uri("/account/identification")
        .insert_header(TestAuthHeader::new(
            response_body.user_id,
            response_body.email,
        ))
        .to_request();

    let response = test::call_service(&app, request).await;
    let response_body: UserIdentificationInfo = read_body_json(response).await;

    assert_eq!(
        response_body.applied_tags.len(),
        1,
        "User should have one tag"
    );
    assert_eq!(
        response_body.applied_tags[0].id, "present-on-event",
        "Tag ID should match"
    );
    assert_eq!(
        response_body.applied_tags[0].name, "Present on event",
        "Tag name should match"
    );
}
