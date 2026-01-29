use chrono::{DateTime, NaiveDateTime};
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{flag_capture, teams};
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::points_counter::{LeaderboardChart, PointsCounter};
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
    PointsCounter::calculate(&Arc::new(app_state))
        .await
        .unwrap()
}

#[tokio::test]
async fn chart() {
    let chart_data = init().await.to_chart();

    let expected = LeaderboardChart {
        event_timestamps: vec![
            date_time_from_str("1970-01-01T00:00:00+00:00"),
            date_time_from_str("2025-02-15T08:30:00+01:00"),
            date_time_from_str("2025-02-15T09:30:00+01:00"),
            date_time_from_str("2025-02-15T09:45:00+01:00"),
            date_time_from_str("2025-02-15T09:50:00+01:00"),
        ],
        team_points_over_time: vec![
            hack4krak_backend::utils::points_counter::TeamPointsTimeSeries {
                name: "one".to_string(),
                color: "#00FF00".to_string(),
                points: vec![0, 0, 0, 500, 300],
            },
            hack4krak_backend::utils::points_counter::TeamPointsTimeSeries {
                name: "zero".to_string(),
                color: "#FF0000".to_string(),
                points: vec![0, 500, 1000, 1000, 800],
            },
            hack4krak_backend::utils::points_counter::TeamPointsTimeSeries {
                name: "two".to_string(),
                color: "#0000FF".to_string(),
                points: vec![0, 0, 0, 0, 300],
            },
            hack4krak_backend::utils::points_counter::TeamPointsTimeSeries {
                name: "three".to_string(),
                color: "#00FFFF".to_string(),
                points: vec![0, 0, 0, 0, 0],
            },
        ],
    };

    let mut chart_data_sorted = chart_data.team_points_over_time;
    chart_data_sorted.sort_by(|a, b| a.name.cmp(&b.name));

    let mut expected_sorted = expected.team_points_over_time;
    expected_sorted.sort_by(|a, b| a.name.cmp(&b.name));

    assert_eq!(chart_data.event_timestamps, expected.event_timestamps);
    assert_eq!(chart_data_sorted, expected_sorted);
}

#[tokio::test]
async fn tie_breakers() {
    // There is a very small chance the test will pass with the wrong order
    // to avoid flaky tests, we run it multiple times
    for _ in 0..100 {
        let rankings = init().await.get_rankings();

        assert_eq!(rankings[0].team_name, "zero");
        assert_eq!(rankings[1].team_name, "one");
        assert_eq!(rankings[2].team_name, "two");
        assert_eq!(rankings[3].team_name, "three");
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
    let points_counter = PointsCounter::calculate(&Arc::new(app_state))
        .await
        .unwrap();
    let results = points_counter.get_rankings();

    // Both teams have same points (1000) and same last solve time (10:00)
    // Team A should rank higher because their first solve was earlier (08:00 vs 09:00)
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].team_name, "team_a");
    assert_eq!(results[0].current_points, 1000);
    assert_eq!(results[1].team_name, "team_b");
    assert_eq!(results[1].current_points, 1000);
}

#[tokio::test]
async fn no_captures() {
    // Test with no flag captures at all
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<flag_capture::Model, Vec<flag_capture::Model>, Vec<Vec<flag_capture::Model>>>(vec![vec![]])
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
    let points_counter = PointsCounter::calculate(&Arc::new(app_state))
        .await
        .unwrap();
    let results = points_counter.get_rankings();

    // All teams should have 0 points and 0 flags
    assert_eq!(results.len(), 2);
    assert_eq!(results[0].current_points, 0);
    assert_eq!(results[0].captured_flags, 0);
    assert_eq!(results[1].current_points, 0);
    assert_eq!(results[1].captured_flags, 0);
}

#[tokio::test]
async fn single_team_multiple_solves() {
    // Test with one team solving multiple tasks
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[
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
                submitted_at: date_time_from_str("2025-02-15T09:00:00+01:00"),
            },
            flag_capture::Model {
                id: 2,
                team: Uuid::from_u128(0),
                task: "task3".to_string(),
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
    let points_counter = PointsCounter::calculate(&Arc::new(app_state))
        .await
        .unwrap();
    let results = points_counter.get_rankings();

    // Team A should have 3 solves and 1500 points (all first solves = 500 each)
    assert_eq!(results[0].team_name, "team_a");
    assert_eq!(results[0].current_points, 1500);
    assert_eq!(results[0].captured_flags, 3);

    // Team B should have 0 solves and 0 points
    assert_eq!(results[1].team_name, "team_b");
    assert_eq!(results[1].current_points, 0);
    assert_eq!(results[1].captured_flags, 0);
}

#[tokio::test]
async fn cache_works() {
    // Test that caching works correctly
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[flag_capture::Model {
            id: 0,
            team: Uuid::from_u128(0),
            task: "task1".to_string(),
            submitted_at: date_time_from_str("2025-02-15T08:00:00+01:00"),
        }]])
        .append_query_results([[teams::Model {
            name: "team_a".to_string(),
            created_at: Default::default(),
            id: Uuid::from_u128(0),
            confirmation_code: None,
            status: TeamStatus::Absent,
            color: "#FF0000".to_string(),
            organization: None,
        }]])
        .into_connection();

    let app_state = Arc::new(AppState::with_database(database));

    // First call - should populate cache
    let result1 = PointsCounter::calculate(&app_state).await.unwrap();
    let rankings1 = result1.get_rankings();

    // Second call - should use cache (same database won't be queried again)
    let result2 = PointsCounter::calculate(&app_state).await.unwrap();
    let rankings2 = result2.get_rankings();

    // Results should be identical
    assert_eq!(rankings1.len(), rankings2.len());
    assert_eq!(rankings1[0].team_name, rankings2[0].team_name);
    assert_eq!(rankings1[0].current_points, rankings2[0].current_points);

    // Invalidate cache
    app_state.invalidate_points_cache().await;

    // Cache should now be empty
    let cache = app_state.points_cache.read().await;
    assert!(cache.is_none());
}

#[tokio::test]
async fn get_team_rank_works() {
    // Test the get_team_rank method
    let points_counter = init().await;

    let rank = points_counter.team_rank("zero");
    assert_eq!(rank, Some((1, 4))); // Rank 1 out of 4 teams

    let rank = points_counter.team_rank("one");
    assert_eq!(rank, Some((2, 4))); // Rank 2 out of 4 teams

    let rank = points_counter.team_rank("nonexistent");
    assert_eq!(rank, None); // Team doesn't exist
}

#[tokio::test]
async fn to_chart_conversion() {
    // Test that to_chart produces correct chart data format
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[flag_capture::Model {
            id: 0,
            team: Uuid::from_u128(0),
            task: "task1".to_string(),
            submitted_at: date_time_from_str("2025-02-15T08:00:00+01:00"),
        }]])
        .append_query_results([[teams::Model {
            name: "team_a".to_string(),
            created_at: Default::default(),
            id: Uuid::from_u128(0),
            confirmation_code: None,
            status: TeamStatus::Absent,
            color: "#FF0000".to_string(),
            organization: None,
        }]])
        .into_connection();

    let app_state = AppState::with_database(database);
    let points_counter = PointsCounter::calculate(&Arc::new(app_state))
        .await
        .unwrap();
    let chart = points_counter.to_chart();

    assert_eq!(chart.event_timestamps.len(), 2);
    assert_eq!(chart.team_points_over_time.len(), 1);
    assert_eq!(chart.team_points_over_time[0].name, "team_a");
    assert_eq!(chart.team_points_over_time[0].points, vec![0, 500]);
    assert_eq!(chart.team_points_over_time[0].color, "#FF0000");
}

#[tokio::test]
async fn empty_counter() {
    // Test behavior with no teams at all
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results::<flag_capture::Model, Vec<flag_capture::Model>, Vec<Vec<flag_capture::Model>>>(vec![vec![]])
        .append_query_results::<teams::Model, Vec<teams::Model>, Vec<Vec<teams::Model>>>(vec![vec![]])
        .into_connection();

    let app_state = AppState::with_database(database);
    let points_counter = PointsCounter::calculate(&Arc::new(app_state))
        .await
        .unwrap();

    let rankings = points_counter.get_rankings();
    assert_eq!(rankings.len(), 0);

    assert_eq!(points_counter.team_rank("any_team"), None);

    let chart = points_counter.to_chart();
    assert_eq!(chart.event_timestamps.len(), 1); // Start time only
    assert_eq!(chart.team_points_over_time.len(), 0);
}

#[tokio::test]
async fn ranking_deep_timestamp_recursion() {
    // Test deep recursive timestamp comparison
    // Teams have same points and same last 2 timestamps, differ on 3rd
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[
            // Team A: solves at 08:00, 09:00, 10:00, 11:00
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
                submitted_at: date_time_from_str("2025-02-15T09:00:00+01:00"),
            },
            flag_capture::Model {
                id: 2,
                team: Uuid::from_u128(0),
                task: "task3".to_string(),
                submitted_at: date_time_from_str("2025-02-15T10:00:00+01:00"),
            },
            flag_capture::Model {
                id: 3,
                team: Uuid::from_u128(0),
                task: "task4".to_string(),
                submitted_at: date_time_from_str("2025-02-15T11:00:00+01:00"),
            },
            // Team B: solves at 08:30, 09:00, 10:00, 11:00
            // Same last 3 timestamps as A (09:00, 10:00, 11:00), but first is later
            flag_capture::Model {
                id: 4,
                team: Uuid::from_u128(1),
                task: "task1".to_string(),
                submitted_at: date_time_from_str("2025-02-15T08:30:00+01:00"),
            },
            flag_capture::Model {
                id: 5,
                team: Uuid::from_u128(1),
                task: "task2".to_string(),
                submitted_at: date_time_from_str("2025-02-15T09:00:00+01:00"),
            },
            flag_capture::Model {
                id: 6,
                team: Uuid::from_u128(1),
                task: "task3".to_string(),
                submitted_at: date_time_from_str("2025-02-15T10:00:00+01:00"),
            },
            flag_capture::Model {
                id: 7,
                team: Uuid::from_u128(1),
                task: "task4".to_string(),
                submitted_at: date_time_from_str("2025-02-15T11:00:00+01:00"),
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
    let points_counter = PointsCounter::calculate(&Arc::new(app_state))
        .await
        .unwrap();
    let rankings = points_counter.get_rankings();

    // Both teams have same points (2000) and same last 3 solve times
    // Team A should rank higher because their FIRST solve was earlier (08:00 vs 08:30)
    // This tests that the algorithm recursively compares ALL timestamps from latest to earliest
    assert_eq!(rankings.len(), 2);
    assert_eq!(
        rankings[0].team_name, "team_a",
        "Team A should rank higher due to earlier first solve (08:00 vs 08:30)"
    );
    assert_eq!(rankings[0].current_points, 2000);
    assert_eq!(rankings[1].team_name, "team_b");
    assert_eq!(rankings[1].current_points, 2000);
}
