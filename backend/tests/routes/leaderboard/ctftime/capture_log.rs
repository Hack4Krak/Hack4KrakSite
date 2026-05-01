use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use hack4krak_backend::entities::teams::UpdatableModel;
use hack4krak_backend::utils::ctftime::CaptureLogEvent;

#[actix_web::test]
async fn capture_log() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let team2 = test_database
        .with_team(UpdatableModel {
            name: Some("test team 2".to_string()),
            ..Default::default()
        })
        .await;
    let team3 = test_database
        .with_team(UpdatableModel {
            name: Some("test team 3".to_string()),
            ..Default::default()
        })
        .await;
    test_database
        .with_flag_capture(&team, "simple-task-example".parse().unwrap())
        .await;
    test_database
        .with_flag_capture(&team2, "task2".parse().unwrap())
        .await;
    test_database
        .with_flag_capture(&team, "task2".parse().unwrap())
        .await;
    test_database
        .with_flag_capture(&team3, "task2".parse().unwrap())
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/ctftime/capture-log?lastId=2147483647")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
    assert_eq!(test::read_body(response).await, "[]");

    let request = test::TestRequest::get()
        .uri("/leaderboard/ctftime/capture-log?lastId=0")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
    let body = test::read_body(response).await;
    assert!(!body.is_empty());
    let events: Vec<CaptureLogEvent> = serde_json::from_slice(&body).unwrap();
    println!("{:?}", events);
    assert_eq!(events.len(), 4);

    let event = &events[0];
    assert_eq!(event.r#type, Some("taskCorrect".to_string()));
    assert_eq!(event.team, "Dziengiel");
    assert_eq!(event.task, Some("simple-task-example".to_string()));

    let event = &events[1];
    assert_eq!(event.r#type, Some("taskCorrect".to_string()));
    assert_eq!(event.team, "test team 2");
    assert_eq!(event.task, Some("task2".to_string()));

    let event = &events[2];
    assert_eq!(event.r#type, Some("taskCorrect".to_string()));
    assert_eq!(event.team, "Dziengiel");
    assert_eq!(event.task, Some("task2".to_string()));

    let event = &events[3];
    assert_eq!(event.r#type, Some("taskCorrect".to_string()));
    assert_eq!(event.team, "test team 3");
    assert_eq!(event.task, Some("task2".to_string()));
}
