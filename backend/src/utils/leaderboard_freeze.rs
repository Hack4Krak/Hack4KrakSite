use chrono::{DateTime, Duration, FixedOffset, NaiveDateTime, Utc};

const LEADERBOARD_FREEZE_BEFORE_END: Duration = Duration::hours(1);

pub fn active_freeze_cutoff(
    now: DateTime<Utc>,
    event_end_date: DateTime<FixedOffset>,
) -> Option<NaiveDateTime> {
    let event_end_utc = event_end_date.with_timezone(&Utc);
    let freeze_start_utc = event_end_utc - LEADERBOARD_FREEZE_BEFORE_END;

    (freeze_start_utc <= now && now < event_end_utc).then_some(freeze_start_utc.naive_utc())
}
