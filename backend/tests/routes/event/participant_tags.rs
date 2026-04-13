use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::task_manager::create_default_test_task_manager;
use actix_web::test;
use actix_web::test::read_body_json;
use hack4krak_backend::models::task_manager::participant_tags_config::ParticipantTag;
use hack4krak_backend::services::task_manager::TaskManager;

#[actix_web::test]
async fn get_participant_tags_success() {
    let test_database = TestDatabase::new().await;
    let task_manager = create_default_test_task_manager().await;

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/event/participant-tags")
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let body: Vec<ParticipantTag> = read_body_json(response).await;
    assert_eq!(body.len(), 2);

    // Verify all tags are present with correct data
    let sponsor = body.iter().find(|t| t.id == "present-on-event").unwrap();
    assert_eq!(sponsor.name, "Present on event");
    assert_eq!(sponsor.description, "Participant is present on event");
    assert_eq!(sponsor.tag_type, "verified");

    let volunteer = body.iter().find(|t| t.id == "breakfast-day-1").unwrap();
    assert_eq!(volunteer.name, "Breakfast day 1");
    assert_eq!(
        volunteer.description,
        "Participant received breakfast on day 1"
    );
    assert_eq!(volunteer.tag_type, "meal");
}

#[actix_web::test]
async fn get_participant_tags_empty() {
    let test_database = TestDatabase::new().await;
    let task_manager = TaskManager::default(); // No tags configured

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/event/participant-tags")
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let body: Vec<ParticipantTag> = read_body_json(response).await;
    assert_eq!(body.len(), 0);
}
