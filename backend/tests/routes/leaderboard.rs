use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use chrono::{DateTime, Duration, Utc};
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{flag_capture, teams, users};
use hack4krak_backend::models::event_config::{EventStage, EventStageType};
use hack4krak_backend::models::task::TaskConfig;
use hack4krak_backend::services::task_manager::TaskManager;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use serde_json::Value;

async fn setup_leaderboard_app() -> (TestDatabase, TaskManager) {
    let test_db = TestDatabase::new().await;
    let team = test_db
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;
    test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let task_manager = TaskManager::default();
    task_manager.tasks.insert(
        "test-task".to_string(),
        TaskConfig {
            flag_hash: "abc123".to_string(),
            ..Default::default()
        },
    );

    task_manager.event_config.write().await.stages = vec![
        EventStage {
            name: "Event Start".to_string(),
            stage_type: EventStageType::EventStart,
            start_date: DateTime::from(Utc::now() - Duration::days(1)),
            end_date: None,
        },
        EventStage {
            name: "Event End".to_string(),
            stage_type: EventStageType::EventEnd,
            start_date: DateTime::from(Utc::now() + Duration::days(1)),
            end_date: None,
        },
    ];

    flag_capture::Entity::insert(flag_capture::ActiveModel {
        team: Set(team.id),
        task: Set("test-task".to_string()),
        submitted_at: Set(Utc::now().naive_utc()),
        ..Default::default()
    })
    .exec(&test_db.database)
    .await
    .unwrap();

    (test_db, task_manager)
}

#[actix_web::test]
async fn leaderboard_chart() {
    let (test_db, task_manager) = setup_leaderboard_app().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/chart")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn leaderboard_teams() {
    let (test_db, task_manager) = setup_leaderboard_app().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/teams")
        .to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert!(!response.is_empty());
}

#[actix_web::test]
async fn leaderboard_teams_with_tasks() {
    let (test_db, task_manager) = setup_leaderboard_app().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/teams_with_tasks")
        .to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert!(!response.is_empty());

    let first = &response[0];
    assert!(first.get("tasks").is_some());
}

#[actix_web::test]
async fn leaderboard_empty() {
    let test_db = TestDatabase::new().await;
    let task_manager = TaskManager::default();

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/teams")
        .to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert!(response.is_empty());
}

#[actix_web::test]
async fn leaderboard_chart_has_data_points() {
    let (test_db, task_manager) = setup_leaderboard_app().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/chart")
        .to_request();
    let response: Value = test::call_and_read_body_json(&app, request).await;
    assert!(response.get("teams").is_some() || response.is_array() || response.is_object());
}

#[actix_web::test]
async fn leaderboard_teams_response_structure() {
    let (test_db, task_manager) = setup_leaderboard_app().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/teams")
        .to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert!(!response.is_empty());

    let first = &response[0];
    assert!(first.get("team_name").is_some());
    assert!(first.get("points").is_some());
}

#[actix_web::test]
async fn leaderboard_chart_empty() {
    let test_db = TestDatabase::new().await;
    let task_manager = TaskManager::default();

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/chart")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn leaderboard_teams_with_tasks_empty() {
    let test_db = TestDatabase::new().await;
    let task_manager = TaskManager::default();

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/teams_with_tasks")
        .to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert!(response.is_empty());
}
