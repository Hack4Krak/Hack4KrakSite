use chrono::DateTime;
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{flag_capture, teams};
use hack4krak_backend::utils::points_counter::{Chart, PointsCounter};
use sea_orm::{DatabaseBackend, MockDatabase};
use serde_json::from_str;
use uuid::Uuid;

#[tokio::test]
async fn count_points() {
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([[
            flag_capture::Model {
                id: 0,
                team: Uuid::from_u128(0),
                task: "first".to_string(),
                submitted_at: DateTime::parse_from_str("2025-02-15T08:30:00+01:00", "%+")
                    .unwrap()
                    .naive_utc(),
            },
            flag_capture::Model {
                id: 1,
                team: Uuid::from_u128(0),
                task: "second".to_string(),
                submitted_at: DateTime::parse_from_str("2025-02-15T09:30:00+01:00", "%+")
                    .unwrap()
                    .naive_utc(),
            },
            flag_capture::Model {
                id: 2,
                team: Uuid::from_u128(1),
                task: "first".to_string(),
                submitted_at: DateTime::parse_from_str("2025-02-15T09:45:00+01:00", "%+")
                    .unwrap()
                    .naive_utc(),
            },
            flag_capture::Model {
                id: 3,
                team: Uuid::from_u128(2),
                task: "first".to_string(),
                submitted_at: DateTime::parse_from_str("2025-02-15T09:50:00+01:00", "%+")
                    .unwrap()
                    .naive_utc(),
            },
        ]])
        .append_query_results([[
            teams::Model {
                name: "zero".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(0),
                confirmation_code: None,
                status: TeamStatus::Absent,
            },
            teams::Model {
                name: "one".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(1),
                confirmation_code: None,
                status: TeamStatus::Absent,
            },
            teams::Model {
                name: "two".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(2),
                confirmation_code: None,
                status: TeamStatus::Absent,
            },
            teams::Model {
                name: "three".to_string(),
                created_at: Default::default(),
                id: Uuid::from_u128(2),
                confirmation_code: None,
                status: TeamStatus::Absent,
            },
        ]])
        .into_connection();

    let chart_data = PointsCounter::work(&database).await.unwrap().to_chart();

    let expected: Chart = from_str(
        r#"
    {
      "event_timestamps": [
        "2025-02-15T07:30:00",
        "2025-02-15T08:30:00",
        "2025-02-15T08:45:00",
        "2025-02-15T08:50:00"
      ],
      "team_points_over_time": [
        {
          "label": "one",
          "points": [
            0,
            0,
            500,
            300
          ]
        },
        {
          "label": "zero",
          "points": [
            500,
            1000,
            1000,
            800
          ]
        },
        {
          "label": "two",
          "points": [
            0,
            0,
            0,
            300
          ]
        },
        {
          "label": "three",
          "points": [
            0,
            0,
            0,
            300
          ]
        }
      ]
    }
    "#,
    )
    .unwrap();

    let mut chart_data_sorted = chart_data.team_points_over_time;
    chart_data_sorted.sort_by(|a, b| a.label.cmp(&b.label));

    let mut expected_sorted = expected.team_points_over_time;
    expected_sorted.sort_by(|a, b| a.label.cmp(&b.label));

    assert_eq!(chart_data.event_timestamps, expected.event_timestamps);
    assert_eq!(chart_data_sorted, expected_sorted);
}
