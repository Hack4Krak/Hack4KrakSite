use chrono::{DateTime, Utc};
use hack4krak_backend::utils::leaderboard_freeze::active_freeze_cutoff;

#[test]
fn returns_cutoff_during_last_hour() {
    let event_end_date =
        DateTime::parse_from_rfc3339("2026-03-17T12:00:00+00:00").expect("failed to parse end date");
    let now = DateTime::parse_from_rfc3339("2026-03-17T11:30:00+00:00")
        .expect("failed to parse now date")
        .with_timezone(&Utc);

    let result = active_freeze_cutoff(now, event_end_date);

    assert_eq!(
        result,
        Some(
            DateTime::parse_from_rfc3339("2026-03-17T11:00:00+00:00")
                .expect("failed to parse expected cutoff")
                .naive_utc(),
        ),
    );
}

#[test]
fn does_not_freeze_before_last_hour() {
    let event_end_date =
        DateTime::parse_from_rfc3339("2026-03-17T12:00:00+00:00").expect("failed to parse end date");
    let now = DateTime::parse_from_rfc3339("2026-03-17T10:59:59+00:00")
        .expect("failed to parse now date")
        .with_timezone(&Utc);

    let result = active_freeze_cutoff(now, event_end_date);

    assert_eq!(result, None);
}

#[test]
fn does_not_freeze_after_event_end() {
    let event_end_date =
        DateTime::parse_from_rfc3339("2026-03-17T12:00:00+00:00").expect("failed to parse end date");
    let now = DateTime::parse_from_rfc3339("2026-03-17T12:00:00+00:00")
        .expect("failed to parse now date")
        .with_timezone(&Utc);

    let result = active_freeze_cutoff(now, event_end_date);

    assert_eq!(result, None);
}
