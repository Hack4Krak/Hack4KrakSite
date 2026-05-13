use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use crate::test_utils::task_manager::create_task_manager_with_default_tasks;
use actix_web::test;
use actix_web::test::TestRequest;
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::users;
use hack4krak_backend::models::announcement::TaskStatus;
use hack4krak_backend::services::task_manager::TaskWithStatus;
use serde_json::json;

#[actix_web::test]
async fn test_announcements_e2e_flow() {
    let test_database = TestDatabase::new().await;

    let admin_user = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let service = TestApp::default()
        .with_database(test_database)
        .with_task_manager(create_task_manager_with_default_tasks())
        .build_app()
        .await;

    let payload = json!({
        "type": "normal",
        "text": "Welcome to Hack4Krak!"
    });

    let request = TestRequest::post()
        .uri("/admin/announcements/")
        .insert_header(TestAuthHeader::new(admin_user.id, admin_user.email.clone()))
        .set_json(&payload)
        .to_request();
    let response = test::call_service(&service, request).await;
    assert!(response.status().is_success());

    let request = TestRequest::get().uri("/announcements/latest").to_request();
    let response = test::call_service(&service, request).await;
    assert!(response.status().is_success());
    let body: serde_json::Value = test::read_body_json(response).await;
    assert_eq!(body["action_type"], "normal");
    assert_eq!(body["additional_data"]["text"], "Welcome to Hack4Krak!");

    let payload = json!({
        "type": "task_status_update",
        "task_id": "simple-task-example",
        "status": "broken",
        "comment": "We are fixing it"
    });

    let request = TestRequest::post()
        .uri("/admin/announcements/")
        .insert_header(TestAuthHeader::new(admin_user.id, admin_user.email.clone()))
        .set_json(&payload)
        .to_request();
    let response = test::call_service(&service, request).await;
    assert!(response.status().is_success());

    let request = TestRequest::get().uri("/tasks/list").to_request();
    let response = test::call_service(&service, request).await;
    assert!(response.status().is_success());
    let body: Vec<TaskWithStatus> = test::read_body_json(response).await;
    assert_eq!(body[0].status, TaskStatus::Broken);
    assert_eq!(body[0].task.description.id, "simple-task-example");
}

#[actix_web::test]
async fn test_announcements_cache_invalidated() {
    let test_database = TestDatabase::new().await;
    let admin_user = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let service = TestApp::default()
        .with_database(test_database)
        .with_task_manager(create_task_manager_with_default_tasks())
        .build_app()
        .await;

    let payload = json!({
        "type": "task_status_update",
        "task_id": "simple-task-example",
        "status": "broken",
        "comment": "We are fixing it"
    });

    let request = TestRequest::post()
        .uri("/admin/announcements/")
        .insert_header(TestAuthHeader::new(admin_user.id, admin_user.email.clone()))
        .set_json(&payload)
        .to_request();

    test::call_service(&service, request).await;

    let request = TestRequest::get().uri("/tasks/list").to_request();

    test::call_service(&service, request).await;

    let payload = json!({
        "type": "task_status_update",
        "task_id": "simple-task-example",
        "status": "up",
        "comment": "We are fixing it"
    });

    let request = TestRequest::post()
        .uri("/admin/announcements/")
        .insert_header(TestAuthHeader::new(admin_user.id, admin_user.email.clone()))
        .set_json(&payload)
        .to_request();

    test::call_service(&service, request).await;

    let request = TestRequest::get().uri("/tasks/list").to_request();

    let response = test::call_service(&service, request).await;
    let body: Vec<TaskWithStatus> = test::read_body_json(response).await;
    assert!(body.iter().all(|task| task.status != TaskStatus::Broken));
}
