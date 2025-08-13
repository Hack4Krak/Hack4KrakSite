use chrono::{DateTime, NaiveDateTime};
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{flag_capture, teams};
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::points_counter::{Chart, PointsCounter};
use sea_orm::{DatabaseBackend, MockDatabase};
use std::sync::Arc;
use uuid::Uuid;

fn date_time_from_str(date: &str) -> NaiveDateTime {
    DateTime::parse_from_str(date, "%+")
        .expect("Failed to parse date time")
        .naive_utc()
}

async fn init() -> PointsCounter {
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[
            flag_capture::Model {
                id: 0,
                team: Uuid::from_u128(0),
                task: "first".to_string(),
                submitted_at: date_time_from_str("2025-02-15T08:30:00+01:00"),
            },
            flag_capture::Model {
                id: 1,
                team: Uuid::from_u128(0),
                task: "second".to_string(),
                submitted_at: date_time_from_str("2025-02-15T09:30:00+01:00"),
            },
            flag_capture::Model {
                id: 2,
                team: Uuid::from_u128(1),
                task: "first".to_string(),
                submitted_at: date_time_from_str("2025-02-15T09:45:00+01:00"),
            },
            flag_capture::Model {
                id: 3,
                team: Uuid::from_u128(2),
                task: "first".to_string(),
                submitted_at: date_time_from_str("2025-02-15T09:50:00+01:00"),
            },
        ]])
        .append_query_results([[
            teams::Model {
                name: "zero".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(0),
                confirmation_code: None,
                status: TeamStatus::Absent,
                color: "#FF0000".to_string(),
                organization: None,
            },
            teams::Model {
                name: "one".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(1),
                confirmation_code: None,
                status: TeamStatus::Absent,
                color: "#00FF00".to_string(),
                organization: None,
            },
            teams::Model {
                name: "two".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(2),
                confirmation_code: None,
                status: TeamStatus::Absent,
                color: "#0000FF".to_string(),
                organization: None,
            },
            teams::Model {
                name: "three".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(3),
                confirmation_code: None,
                status: TeamStatus::Absent,
                color: "#00FFFF".to_string(),
                organization: None,
            },
        ]])
        .into_connection();

    let app_state = AppState::with_database(database);
    PointsCounter::work(&Arc::new(app_state)).await.unwrap()
}

#[tokio::test]
async fn chart() {
    let chart_data = init().await.to_chart();

    let expected = Chart {
        event_timestamps: vec![
            date_time_from_str("1970-01-01T00:00:00+00:00"),
            date_time_from_str("2025-02-15T08:30:00+01:00"),
            date_time_from_str("2025-02-15T09:30:00+01:00"),
            date_time_from_str("2025-02-15T09:45:00+01:00"),
            date_time_from_str("2025-02-15T09:50:00+01:00"),
        ],
        team_points_over_time: vec![
            hack4krak_backend::utils::points_counter::TeamPoints {
                label: "one".to_string(),
                color: "#00FF00".to_string(),
                points: vec![0, 0, 0, 500, 300],
            },
            hack4krak_backend::utils::points_counter::TeamPoints {
                label: "zero".to_string(),
                color: "#FF0000".to_string(),
                points: vec![0, 500, 1000, 1000, 800],
            },
            hack4krak_backend::utils::points_counter::TeamPoints {
                label: "two".to_string(),
                color: "#0000FF".to_string(),
                points: vec![0, 0, 0, 0, 300],
            },
            hack4krak_backend::utils::points_counter::TeamPoints {
                label: "three".to_string(),
                color: "#00FFFF".to_string(),
                points: vec![0, 0, 0, 0, 0],
            },
        ],
    };

    let mut chart_data_sorted = chart_data.team_points_over_time;
    chart_data_sorted.sort_by(|a, b| a.label.cmp(&b.label));

    let mut expected_sorted = expected.team_points_over_time;
    expected_sorted.sort_by(|a, b| a.label.cmp(&b.label));

    assert_eq!(chart_data.event_timestamps, expected.event_timestamps);
    assert_eq!(chart_data_sorted, expected_sorted);
}

#[tokio::test]
async fn tie_breakers() {
    // There is a very small chance the test will pass with the wrong order
    // to avoid flaky tests, we run it multiple times
    for _ in 0..100 {
        let chart_data = init().await.get_final_team_points();

        assert_eq!(chart_data[0].team_name, "zero");
        assert_eq!(chart_data[1].team_name, "one");
        assert_eq!(chart_data[2].team_name, "two");
        assert_eq!(chart_data[3].team_name, "three");
    }
}

#[tokio::test]
async fn tie_breakers_multi_level() {
    // Test scenario where teams have same points and same last solve time
    // Should fall back to comparing earlier solve times
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[
            // Team A: solves at 08:00, 10:00 (total: 2 tasks)
            flag_capture::Model {
                id: 0,
                team: Uuid::from_u128(0),
                task: "task1".to_string(),
                submitted_at: date_time_from_str("2025-02-15T08:00:00+01:00"),
            },
            flag_capture::Model {
                id: 1,
                team: Uuid::from_u128(0),
                task: "task2".to_string(),
                submitted_at: date_time_from_str("2025-02-15T10:00:00+01:00"),
            },
            // Team B: solves at 09:00, 10:00 (total: 2 tasks, same last time as A)
            flag_capture::Model {
                id: 2,
                team: Uuid::from_u128(1),
                task: "task1".to_string(),
                submitted_at: date_time_from_str("2025-02-15T09:00:00+01:00"),
            },
            flag_capture::Model {
                id: 3,
                team: Uuid::from_u128(1),
                task: "task2".to_string(),
                submitted_at: date_time_from_str("2025-02-15T10:00:00+01:00"),
            },
        ]])
        .append_query_results([[
            teams::Model {
                name: "team_a".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(0),
                confirmation_code: None,
                status: TeamStatus::Absent,
                color: "#FF0000".to_string(),
                organization: None,
            },
            teams::Model {
                name: "team_b".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(1),
                confirmation_code: None,
                status: TeamStatus::Absent,
                color: "#00FF00".to_string(),
                organization: None,
            },
        ]])
        .into_connection();

    let app_state = AppState::with_database(database);
    let points_counter = PointsCounter::work(&Arc::new(app_state)).await.unwrap();
    let results = points_counter.get_final_team_points();

    // Both teams have same points (1000) and same last solve time (10:00)
    // Team A should rank higher because their first solve was earlier (08:00 vs 09:00)
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].team_name, "team_a");
    assert_eq!(results[0].current_points, 1000);
    assert_eq!(results[1].team_name, "team_b");
    assert_eq!(results[1].current_points, 1000);
}
