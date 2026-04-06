use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::task_manager::create_default_test_task_manager;
use actix_web::test;
use actix_web::test::read_body_json;
use hack4krak_backend::models::task::ParticipantTag;
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
    assert_eq!(body.len(), 3);

    // Verify all tags are present with correct data
    let sponsor = body.iter().find(|t| t.id == "sponsor").unwrap();
    assert_eq!(sponsor.name, "Sponsor");
    assert_eq!(sponsor.description, "Event sponsor");

    let volunteer = body.iter().find(|t| t.id == "volunteer").unwrap();
    assert_eq!(volunteer.name, "Volunteer");
    assert_eq!(volunteer.description, "Event volunteer");

    let judge = body.iter().find(|t| t.id == "judge").unwrap();
    assert_eq!(judge.name, "Judge");
    assert_eq!(judge.description, "Event judge");
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
