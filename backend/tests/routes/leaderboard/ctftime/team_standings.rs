use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use hack4krak_backend::entities::teams::UpdatableModel;
use hack4krak_backend::models::task_manager::task_config::TaskConfig;
use hack4krak_backend::services::task_manager::TaskManager;
use hack4krak_backend::utils::ctftime::{SingleTeamStanding, TaskTeamStat, TeamStandings};
use std::collections::HashMap;

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

    let team_capture = test_database
        .with_flag_capture(&team, "simple-task-example".parse().unwrap())
        .await;
    let team2_capture = test_database
        .with_flag_capture(&team2, "task2".parse().unwrap())
        .await;
    let team_task2_capture = test_database
        .with_flag_capture(&team, "task2".parse().unwrap())
        .await;
    let team3_capture = test_database
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

    let mut standings: TeamStandings = serde_json::from_slice(&body).unwrap();
    standings.standings.sort_by(|a, b| a.team.cmp(&b.team));

    let timestamp = |capture: &hack4krak_backend::entities::flag_capture::Model| {
        capture.submitted_at.and_utc().timestamp()
    };
    assert_eq!(
        standings,
        TeamStandings {
            tasks: vec!["simple-task-example".to_string(), "task2".to_string()],
            standings: vec![
                SingleTeamStanding {
                    team: "Dziengiel".to_string(),
                    score: 600,
                    task_stats: HashMap::from([
                        (
                            "simple-task-example".to_string(),
                            TaskTeamStat {
                                points: 500,
                                time: timestamp(&team_capture),
                            },
                        ),
                        (
                            "task2".to_string(),
                            TaskTeamStat {
                                points: 100,
                                time: timestamp(&team_task2_capture),
                            },
                        ),
                    ]),
                    last_accept: timestamp(&team_task2_capture),
                },
                SingleTeamStanding {
                    team: "test team 2".to_string(),
                    score: 100,
                    task_stats: HashMap::from([(
                        "task2".to_string(),
                        TaskTeamStat {
                            points: 100,
                            time: timestamp(&team2_capture),
                        },
                    )]),
                    last_accept: timestamp(&team2_capture),
                },
                SingleTeamStanding {
                    team: "test team 3".to_string(),
                    score: 100,
                    task_stats: HashMap::from([(
                        "task2".to_string(),
                        TaskTeamStat {
                            points: 100,
                            time: timestamp(&team3_capture),
                        },
                    )]),
                    last_accept: timestamp(&team3_capture),
                },
            ],
        }
    );
}
