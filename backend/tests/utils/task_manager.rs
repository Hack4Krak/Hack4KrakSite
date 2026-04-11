use hack4krak_backend::models::event_config::{EventConfig, EventStage, EventStageType};
use hack4krak_backend::models::task::{TaskConfig, TaskMeta};
use hack4krak_backend::services::task_manager::TaskManager;

#[test]
fn get_task_valid_id() {
    let tm = TaskManager::default();
    tm.tasks.insert(
        "valid-task".to_string(),
        TaskConfig {
            meta: TaskMeta {
                id: "valid-task".to_string(),
                name: "Valid Task".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let result = tm.get_task("valid-task");
    assert!(result.is_ok());
    assert_eq!(result.unwrap().meta.name, "Valid Task");
}

#[test]
fn get_task_missing_id() {
    let tm = TaskManager::default();

    let result = tm.get_task("nonexistent");
    assert!(result.is_err());
}

#[test]
fn get_task_invalid_id_with_path_traversal() {
    let tm = TaskManager::default();

    let result = tm.get_task("../../etc/passwd");
    assert!(result.is_err());
}

#[test]
fn get_task_invalid_id_with_special_chars() {
    let tm = TaskManager::default();

    let result = tm.get_task("task@#$%");
    assert!(result.is_err());
}

#[test]
fn get_task_valid_id_with_underscores_and_hyphens() {
    let tm = TaskManager::default();
    tm.tasks
        .insert("task-with_underscores".to_string(), TaskConfig::default());

    let result = tm.get_task("task-with_underscores");
    assert!(result.is_ok());
}

#[test]
fn get_tasks_sorted() {
    let tm = TaskManager::default();
    tm.tasks.insert(
        "b-task".to_string(),
        TaskConfig {
            meta: TaskMeta {
                id: "b-task".to_string(),
                name: "B Task".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    tm.tasks.insert(
        "a-task".to_string(),
        TaskConfig {
            meta: TaskMeta {
                id: "a-task".to_string(),
                name: "A Task".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    let sorted = tm.get_tasks_sorted();
    assert_eq!(sorted.len(), 2);
    assert_eq!(sorted[0].description.id, "a-task");
    assert_eq!(sorted[1].description.id, "b-task");
}

#[test]
fn get_tasks_sorted_empty() {
    let tm = TaskManager::default();

    let sorted = tm.get_tasks_sorted();
    assert!(sorted.is_empty());
}

#[test]
fn event_config_stage_by_type() {
    let config = EventConfig::default();

    let start = config.stage_by_type(&EventStageType::EventStart);
    assert!(start.is_some());
    assert_eq!(start.unwrap().stage_type, EventStageType::EventStart);

    let end = config.stage_by_type(&EventStageType::EventEnd);
    assert!(end.is_some());

    // Informative type returns None
    let info = config.stage_by_type(&EventStageType::Informative);
    assert!(info.is_none());
}

#[test]
fn event_config_stage_by_name() {
    let config = EventConfig::default();

    let found = config.stage_by_name("Event Start");
    assert!(found.is_some());

    let not_found = config.stage_by_name("Nonexistent");
    assert!(not_found.is_none());
}

#[test]
fn event_config_is_after_event_with_past_end() {
    let config = EventConfig {
        stages: vec![
            EventStage {
                name: "Event Start".to_string(),
                stage_type: EventStageType::EventStart,
                start_date: chrono::DateTime::from(chrono::Utc::now() - chrono::Duration::days(2)),
                end_date: None,
            },
            EventStage {
                name: "Event End".to_string(),
                stage_type: EventStageType::EventEnd,
                start_date: chrono::DateTime::from(chrono::Utc::now() - chrono::Duration::days(1)),
                end_date: None,
            },
        ],
        ..Default::default()
    };

    assert!(config.is_after_event());
    assert!(!config.is_before_event());
    assert!(!config.is_during_event());
}

#[test]
fn event_config_is_before_event() {
    let config = EventConfig {
        stages: vec![
            EventStage {
                name: "Event Start".to_string(),
                stage_type: EventStageType::EventStart,
                start_date: chrono::DateTime::from(chrono::Utc::now() + chrono::Duration::days(1)),
                end_date: None,
            },
            EventStage {
                name: "Event End".to_string(),
                stage_type: EventStageType::EventEnd,
                start_date: chrono::DateTime::from(chrono::Utc::now() + chrono::Duration::days(2)),
                end_date: None,
            },
        ],
        ..Default::default()
    };

    assert!(config.is_before_event());
    assert!(!config.is_after_event());
    assert!(!config.is_during_event());
}

#[test]
fn event_config_is_during_event() {
    let config = EventConfig {
        stages: vec![
            EventStage {
                name: "Event Start".to_string(),
                stage_type: EventStageType::EventStart,
                start_date: chrono::DateTime::from(chrono::Utc::now() - chrono::Duration::days(1)),
                end_date: None,
            },
            EventStage {
                name: "Event End".to_string(),
                stage_type: EventStageType::EventEnd,
                start_date: chrono::DateTime::from(chrono::Utc::now() + chrono::Duration::days(1)),
                end_date: None,
            },
        ],
        ..Default::default()
    };

    assert!(config.is_during_event());
    assert!(!config.is_before_event());
    assert!(!config.is_after_event());
}
