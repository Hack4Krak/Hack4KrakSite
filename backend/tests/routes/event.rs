use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use chrono::{DateTime, Duration, Utc};
use hack4krak_backend::models::event_config::{EventStage, EventStageType};
use hack4krak_backend::services::task_manager::TaskManager;
use serde_json::Value;

#[actix_web::test]
async fn event_info() {
    let test_db = TestDatabase::new().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get().uri("/event/info").to_request();
    let response: Value = test::call_and_read_body_json(&app, request).await;
    assert!(response.get("id").is_some());
    assert!(response.get("name").is_some());
    assert!(response.get("stages").is_some());
}

#[actix_web::test]
async fn event_registration() {
    let test_db = TestDatabase::new().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/event/registration")
        .to_request();
    let response: Value = test::call_and_read_body_json(&app, request).await;
    assert!(response.get("max_team_size").is_some());
    assert!(response.get("max_teams").is_some());
}

#[actix_web::test]
async fn event_status_during_event() {
    let test_db = TestDatabase::new().await;
    let task_manager = TaskManager::default();

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

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get().uri("/event/status").to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn event_status_before_event() {
    let test_db = TestDatabase::new().await;
    let task_manager = TaskManager::default();

    task_manager.event_config.write().await.stages = vec![
        EventStage {
            name: "Event Start".to_string(),
            stage_type: EventStageType::EventStart,
            start_date: DateTime::from(Utc::now() + Duration::days(1)),
            end_date: None,
        },
        EventStage {
            name: "Event End".to_string(),
            stage_type: EventStageType::EventEnd,
            start_date: DateTime::from(Utc::now() + Duration::days(2)),
            end_date: None,
        },
    ];

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get().uri("/event/status").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn event_label_not_found() {
    let test_db = TestDatabase::new().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/event/label/nonexistent")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 500);
}

#[actix_web::test]
async fn event_status_after_event() {
    let test_db = TestDatabase::new().await;
    let task_manager = TaskManager::default();

    task_manager.event_config.write().await.stages = vec![
        EventStage {
            name: "Event Start".to_string(),
            stage_type: EventStageType::EventStart,
            start_date: DateTime::from(Utc::now() - Duration::days(2)),
            end_date: None,
        },
        EventStage {
            name: "Event End".to_string(),
            stage_type: EventStageType::EventEnd,
            start_date: DateTime::from(Utc::now() - Duration::days(1)),
            end_date: None,
        },
    ];

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get().uri("/event/status").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 200);
}

#[actix_web::test]
async fn event_info_has_stages() {
    let test_db = TestDatabase::new().await;
    let task_manager = TaskManager::default();

    task_manager.event_config.write().await.stages = vec![
        EventStage {
            name: "Event Start".to_string(),
            stage_type: EventStageType::EventStart,
            start_date: DateTime::from(Utc::now() - Duration::days(1)),
            end_date: None,
        },
        EventStage {
            name: "Lunch Break".to_string(),
            stage_type: EventStageType::Informative,
            start_date: DateTime::from(Utc::now()),
            end_date: Some(DateTime::from(Utc::now() + Duration::hours(1))),
        },
        EventStage {
            name: "Event End".to_string(),
            stage_type: EventStageType::EventEnd,
            start_date: DateTime::from(Utc::now() + Duration::days(1)),
            end_date: None,
        },
    ];

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get().uri("/event/info").to_request();
    let response: Value = test::call_and_read_body_json(&app, request).await;
    let stages = response.get("stages").unwrap().as_array().unwrap();
    assert_eq!(stages.len(), 3);
    assert_eq!(stages[1].get("name").unwrap(), "Lunch Break");
}
