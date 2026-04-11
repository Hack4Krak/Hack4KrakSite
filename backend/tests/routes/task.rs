use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use chrono::{DateTime, Duration, Utc};
use hack4krak_backend::models::event_config::{EventStage, EventStageType};
use hack4krak_backend::models::task::{TaskConfig, TaskMeta};
use hack4krak_backend::services::task_manager::TaskManager;
use serde_json::Value;

fn task_manager_with_tasks() -> TaskManager {
    let task_manager = TaskManager::default();
    task_manager.tasks.insert(
        "task-1".to_string(),
        TaskConfig {
            meta: TaskMeta {
                id: "task-1".to_string(),
                name: "First Task".to_string(),
                ..Default::default()
            },
            flag_hash: "abc".to_string(),
            ..Default::default()
        },
    );
    task_manager.tasks.insert(
        "task-2".to_string(),
        TaskConfig {
            meta: TaskMeta {
                id: "task-2".to_string(),
                name: "Second Task".to_string(),
                ..Default::default()
            },
            flag_hash: "def".to_string(),
            ..Default::default()
        },
    );
    task_manager
}

async fn task_manager_with_event_started() -> TaskManager {
    let tm = task_manager_with_tasks();
    tm.event_config.write().await.stages = vec![
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
    tm
}

#[actix_web::test]
async fn task_list() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_event_started().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get().uri("/tasks/list").to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response.len(), 2);
}

#[actix_web::test]
async fn task_count() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_event_started().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get().uri("/tasks/count").to_request();
    let response: u16 = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response, 2);
}

#[actix_web::test]
async fn task_name_success() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_event_started().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/tasks/name/task-1")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let body = test::read_body(response).await;
    assert_eq!(body, "First Task");
}

#[actix_web::test]
async fn task_name_not_found() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_event_started().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/tasks/name/nonexistent")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn task_name_invalid_id() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_event_started().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/tasks/name/task@invalid!")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn task_list_before_event() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_tasks();

    // Set event to future
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

    let request = test::TestRequest::get().uri("/tasks/list").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn task_count_before_event() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_tasks();

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

    let request = test::TestRequest::get().uri("/tasks/count").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn task_description_not_found() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_event_started().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/tasks/description/nonexistent")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error() || response.status().is_server_error());
}

#[actix_web::test]
async fn task_icon_not_found() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_event_started().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/tasks/icon/nonexistent")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error() || response.status().is_server_error());
}

#[actix_web::test]
async fn task_name_before_event() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_tasks();

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

    let request = test::TestRequest::get()
        .uri("/tasks/name/task-1")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn task_list_response_structure() {
    let test_db = TestDatabase::new().await;
    let task_manager = task_manager_with_event_started().await;

    let app = TestApp::default()
        .with_database(test_db)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get().uri("/tasks/list").to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response.len(), 2);

    let first = &response[0];
    assert!(first.get("id").is_some());
    assert!(first.get("name").is_some());
}

#[actix_web::test]
async fn task_count_empty() {
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

    let request = test::TestRequest::get().uri("/tasks/count").to_request();
    let response: u16 = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response, 0);
}
