use actix_web::web::Bytes;
use hack4krak_backend::utils::server_event::{
    BroadcastEnvelope, EventAudience, LeaderboardUpdateEvent, ServerEvent, ServerEventTopic,
    SubscriberFilter, TeamFlagCaptureEvent,
};
use sea_orm::prelude::Uuid;
use std::collections::HashSet;

fn sample_leaderboard_event() -> ServerEvent {
    ServerEvent::LeaderboardUpdate(LeaderboardUpdateEvent {
        task_id: "task_id".to_string(),
        task_name: "task_name".to_string(),
        is_first_flag_submission: true,
        team_name: "team_name".to_string(),
        username: "username".to_string(),
    })
}

fn sample_team_event() -> ServerEvent {
    ServerEvent::TeamFlagCapture(TeamFlagCaptureEvent {
        task_id: "task_id".to_string(),
        task_name: "task_name".to_string(),
        username: "username".to_string(),
        is_first_flag_submission: false,
    })
}

#[test]
fn to_message_serializes_payload_with_event_header() {
    let message = sample_leaderboard_event().to_message().unwrap();
    let expected = "event: leaderboard_update\ndata: {\"type\":\"leaderboard_update\",\"payload\":{\"task_id\":\"task_id\",\"task_name\":\"task_name\",\"is_first_flag_submission\":true,\"team_name\":\"team_name\",\"username\":\"username\"}}\n\n";
    assert_eq!(message, Bytes::from(expected));
}

#[test]
fn heartbeat_message_matches_sse_comment_format() {
    assert_eq!(
        ServerEvent::heartbeat_message(),
        Bytes::from_static(b": heartbeat\n\n")
    );
}

#[test]
fn topics_serialize_as_snake_case() {
    assert_eq!(
        serde_json::to_string(&ServerEventTopic::Leaderboard).unwrap(),
        "\"leaderboard\""
    );
    assert_eq!(
        serde_json::to_string(&ServerEventTopic::Team).unwrap(),
        "\"team\""
    );
}

#[test]
fn filter_accepts_public_event_on_subscribed_topic() {
    let filter = SubscriberFilter {
        topics: HashSet::from([ServerEventTopic::Leaderboard]),
        team_id: None,
    };
    let envelope = BroadcastEnvelope::public(sample_leaderboard_event());
    assert!(filter.accepts(&envelope));
}

#[test]
fn filter_rejects_event_on_unsubscribed_topic() {
    let filter = SubscriberFilter {
        topics: HashSet::from([ServerEventTopic::Leaderboard]),
        team_id: None,
    };
    let envelope = BroadcastEnvelope::team(Uuid::new_v4(), sample_team_event());
    assert!(!filter.accepts(&envelope));
}

#[test]
fn filter_team_event_requires_matching_team() {
    let team_id = Uuid::new_v4();
    let other_team = Uuid::new_v4();
    let topics = HashSet::from([ServerEventTopic::Team]);
    let envelope = BroadcastEnvelope::team(team_id, sample_team_event());

    let matching = SubscriberFilter {
        topics: topics.clone(),
        team_id: Some(team_id),
    };
    assert!(matching.accepts(&envelope));

    let wrong_team = SubscriberFilter {
        topics: topics.clone(),
        team_id: Some(other_team),
    };
    assert!(!wrong_team.accepts(&envelope));

    let anonymous = SubscriberFilter {
        topics,
        team_id: None,
    };
    assert!(!anonymous.accepts(&envelope));
}

#[test]
fn event_audience_variants_construct() {
    let _public = EventAudience::Public;
    let _team = EventAudience::Team(Uuid::nil());
}
