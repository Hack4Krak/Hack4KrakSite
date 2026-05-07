use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use hack4krak_backend::entities::teams::UpdatableModel;
use hack4krak_backend::models::task::TaskConfig;
use hack4krak_backend::services::task_manager::TaskManager;

#[actix_web::test]
async fn team_standings() {
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

    let task_manager = TaskManager::default();
    task_manager
        .tasks
        .insert("simple-task-example".to_string(), TaskConfig::default());
    task_manager
        .tasks
        .insert("task2".to_string(), TaskConfig::default());

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/leaderboard/ctftime/team-standings")
        .to_request();
    let response = test::call_service(&app, request).await;
    let status = response.status();
    let body = test::read_body(response).await;

    assert!(
        status.is_success(),
        "Expected success but got {}: {:?}",
        status,
        String::from_utf8_lossy(&body)
    );

    let standalone: serde_json::Value = serde_json::from_slice(&body).unwrap();

    assert!(
        !standalone
            .get("tasks")
            .unwrap()
            .as_array()
            .unwrap()
            .is_empty()
    );

    let standings = standalone.get("standings").unwrap().as_array().unwrap();

    assert!(!standings.is_empty());

    let dziengiel_standing = standings
        .iter()
        .find(|s| s.get("team").unwrap().as_str().unwrap() == "Dziengiel")
        .expect("team should be in standings");

    assert!(dziengiel_standing.get("score").unwrap().as_u64().unwrap() > 0);
    assert!(
        !dziengiel_standing
            .get("taskStats")
            .unwrap()
            .as_object()
            .unwrap()
            .is_empty()
    );
    assert!(
        dziengiel_standing
            .get("lastAccept")
            .unwrap()
            .as_i64()
            .unwrap()
            > 0
    );
}
