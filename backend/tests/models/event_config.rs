use chrono::{Duration, Utc};
use hack4krak_backend::models::event_config::{EventConfig, EventStage, EventStageType};

fn make_config(start_offset: Duration, end_offset: Duration) -> EventConfig {
    let now = Utc::now().fixed_offset();
    EventConfig {
        id: "test".to_string(),
        name: "Test Event".to_string(),
        stages: vec![
            EventStage {
                name: "Start".to_string(),
                stage_type: EventStageType::EventStart,
                start_date: now + start_offset,
                end_date: None,
            },
            EventStage {
                name: "End".to_string(),
                stage_type: EventStageType::EventEnd,
                start_date: now + end_offset,
                end_date: None,
            },
        ],
    }
}

#[test]
fn event_stage_start_returns_start_date_for_event_start() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    let start = config.event_stage_start(EventStageType::EventStart);
    assert!(start.is_some());
}

#[test]
fn event_stage_start_returns_start_date_for_event_end() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    let end = config.event_stage_start(EventStageType::EventEnd);
    assert!(end.is_some());
}

#[test]
fn event_stage_start_returns_none_for_informative() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    let result = config.event_stage_start(EventStageType::Informative);
    assert!(result.is_none());
}

#[test]
fn is_before_event_when_start_is_in_future() {
    let config = make_config(Duration::hours(1), Duration::hours(2));
    assert!(config.is_before_event());
}

#[test]
fn is_not_before_event_when_start_has_passed() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    assert!(!config.is_before_event());
}

#[test]
fn is_during_event_when_between_start_and_end() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    assert!(config.is_during_event());
}

#[test]
fn is_not_during_event_when_before_start() {
    let config = make_config(Duration::hours(1), Duration::hours(2));
    assert!(!config.is_during_event());
}

#[test]
fn is_not_during_event_when_after_end() {
    let config = make_config(Duration::hours(-2), Duration::hours(-1));
    assert!(!config.is_during_event());
}

#[test]
fn is_after_event_when_end_has_passed() {
    let config = make_config(Duration::hours(-2), Duration::hours(-1));
    assert!(config.is_after_event());
}

#[test]
fn is_not_after_event_when_end_is_in_future() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    assert!(!config.is_after_event());
}

#[test]
fn stage_by_type_returns_correct_stage() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    let stage = config.stage_by_type(&EventStageType::EventStart).unwrap();
    assert_eq!(stage.name, "Start");
}

#[test]
fn stage_by_type_returns_none_for_informative() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    assert!(config.stage_by_type(&EventStageType::Informative).is_none());
}

#[test]
fn stage_by_name_returns_correct_stage() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    let stage = config.stage_by_name("End").unwrap();
    assert!(matches!(stage.stage_type, EventStageType::EventEnd));
}

#[test]
fn stage_by_name_returns_none_for_missing() {
    let config = make_config(Duration::hours(-1), Duration::hours(1));
    assert!(config.stage_by_name("Nonexistent").is_none());
}

#[test]
fn default_event_config_has_two_stages() {
    let config = EventConfig::default();
    assert_eq!(config.stages.len(), 2);
    assert_eq!(config.id, "tasks");
    assert_eq!(config.name, "Default Event");
}

#[test]
fn default_event_config_has_start_and_end() {
    let config = EventConfig::default();
    assert!(config.stage_by_type(&EventStageType::EventStart).is_some());
    assert!(config.stage_by_type(&EventStageType::EventEnd).is_some());
}

#[test]
fn is_before_event_returns_true_when_no_start_stage() {
    let config = EventConfig {
        id: "test".to_string(),
        name: "Empty".to_string(),
        stages: vec![],
    };
    assert!(config.is_before_event());
}

#[test]
fn is_after_event_returns_false_when_no_end_stage() {
    let config = EventConfig {
        id: "test".to_string(),
        name: "Empty".to_string(),
        stages: vec![],
    };
    assert!(!config.is_after_event());
}

#[test]
fn informative_stage_is_ignored_in_type_lookup() {
    let config = EventConfig {
        id: "test".to_string(),
        name: "Test".to_string(),
        stages: vec![EventStage {
            name: "Info".to_string(),
            stage_type: EventStageType::Informative,
            start_date: Utc::now().fixed_offset(),
            end_date: None,
        }],
    };
    assert!(config.stage_by_type(&EventStageType::Informative).is_none());
    assert!(config.stage_by_name("Info").is_some());
}
