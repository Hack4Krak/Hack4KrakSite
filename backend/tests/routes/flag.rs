use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use actix_web::test::TestRequest;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{teams, users};
use hack4krak_backend::models::task::TaskConfig;
use hack4krak_backend::services::task_manager::TaskManager;
use serde_json::json;

async fn submit_flag(user: users::Model, flag: &str) -> TestRequest {
    TestRequest::post()
        .uri("/flag/submit")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(json!({ "flag": flag }))
}

async fn setup_app_with_task_manager(
    event_start_date: DateTime<FixedOffset>,
    event_end_date: DateTime<FixedOffset>,
) -> (users::Model, users::Model, TestDatabase, TaskManager) {
    let test_database = TestDatabase::new().await;
    let confirmed_team = test_database
        .with_team(teams::UpdatableModel {
            name: Some("team2".to_string()),
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;
    let present_user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(confirmed_team.id)),
            ..Default::default()
        })
        .await;
    let unconfirmed_team = test_database.with_default_team().await;
    let absent_user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(unconfirmed_team.id)),
            ..Default::default()
        })
        .await;

    let task_manager = TaskManager::default();
    task_manager.tasks.insert(
        "test-task".to_string(),
        TaskConfig {
            flag_hash: "1912766d6ba0e50e8b1bacfb51207e83b95b7ac0cd8ce15307cdf4965e7e3f6c"
                .to_string(),
            ..Default::default()
        },
    );

    task_manager.event_config.write().await.start_date = event_start_date;
    task_manager.event_config.write().await.end_date = event_end_date;

    (absent_user, present_user, test_database, task_manager)
}

#[actix_web::test]
async fn try_submitting_flags() {
    let (absent_user, present_user, test_database, task_manager) = setup_app_with_task_manager(
        DateTime::from(Utc::now()),
        DateTime::from(Utc::now() + Duration::days(1)),
    )
    .await;

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = submit_flag(absent_user.clone(), "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 403);

    let request = submit_flag(present_user.clone(), "hack4ka").await;
    let response: serde_json::Value =
        test::call_and_read_body_json(&app, request.to_request()).await;
    assert_eq!(response["error"].as_str().unwrap(), "Flag");

    let request = submit_flag(present_user.clone(), "hack4KrakCTF{...asds}").await;
    let response: serde_json::Value =
        test::call_and_read_body_json(&app, request.to_request()).await;
    assert_eq!(response["error"].as_str().unwrap(), "Flag");

    let request = submit_flag(present_user.clone(), "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 200);

    let request = submit_flag(present_user.clone(), "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn submit_flag_after_event() {
    let (_, present_user, database, task_manager) = setup_app_with_task_manager(
        DateTime::from(Utc::now() - Duration::days(1)),
        DateTime::from(Utc::now() - Duration::days(1)),
    )
    .await;

    let app = TestApp::default()
        .with_database(database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = submit_flag(present_user.clone(), "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 200);
    let (_, present_user, database, task_manager) = setup_app_with_task_manager(
        DateTime::from(Utc::now() - Duration::days(1)),
        DateTime::from(Utc::now() - Duration::days(1)),
    )
    .await;

    let app = TestApp::default()
        .with_database(database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = submit_flag(present_user.clone(), "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 200);
}
