use crate::test_utils::database::TestDatabase;
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{flag_capture, teams};

#[tokio::test]
async fn has_flags_for_task_returns_false_when_empty() {
    let test_db = TestDatabase::new().await;

    let result =
        flag_capture::Model::has_flags_for_task(&test_db.database, "task_1".to_string()).await;
    assert_eq!(result.unwrap(), false);
}

#[tokio::test]
async fn completed_inserts_flag_capture() {
    let test_db = TestDatabase::new().await;
    let team = test_db
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;

    let is_first = flag_capture::Model::completed(&test_db.database, team, "task_1".to_string())
        .await
        .unwrap();
    assert!(!is_first);
}

#[tokio::test]
async fn completed_returns_true_when_first_for_task() {
    let test_db = TestDatabase::new().await;
    let team1 = test_db
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;
    let team2 = test_db
        .with_team(teams::UpdatableModel {
            name: Some("OtherTeam".to_string()),
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;

    flag_capture::Model::completed(&test_db.database, team1, "task_1".to_string())
        .await
        .unwrap();

    let is_first =
        flag_capture::Model::completed(&test_db.database, team2, "task_1".to_string()).await;
    assert!(is_first.unwrap());
}

#[tokio::test]
async fn completed_duplicate_returns_error() {
    let test_db = TestDatabase::new().await;
    let team = test_db
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;

    flag_capture::Model::completed(&test_db.database, team.clone(), "task_1".to_string())
        .await
        .unwrap();

    let result =
        flag_capture::Model::completed(&test_db.database, team, "task_1".to_string()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn has_flags_for_task_returns_true_after_capture() {
    let test_db = TestDatabase::new().await;
    let team = test_db
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;

    flag_capture::Model::completed(&test_db.database, team, "task_1".to_string())
        .await
        .unwrap();

    let result =
        flag_capture::Model::has_flags_for_task(&test_db.database, "task_1".to_string()).await;
    assert!(result.unwrap());
}
