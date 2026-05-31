use chrono::{DateTime, FixedOffset};
use dashmap::DashMap;
use hack4krak_backend::models::task_manager::event_config::EventConfig;
use hack4krak_backend::models::task_manager::participant_tags_config::{
    ParticipantTag, ParticipantTagsConfig,
};
use hack4krak_backend::models::task_manager::task_config::{TaskConfig, TaskMeta};
use hack4krak_backend::services::task_manager::TaskManager;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub fn default_participant_tags() -> Vec<ParticipantTag> {
    vec![
        ParticipantTag {
            id: "present-on-event".to_string(),
            name: "Present on event".to_string(),
            description: "Participant is present on event".to_string(),
            tag_type: "verified".to_string(),
        },
        ParticipantTag {
            id: "breakfast-day-1".to_string(),
            name: "Breakfast day 1".to_string(),
            description: "Participant received breakfast on day 1".to_string(),
            tag_type: "meal".to_string(),
        },
    ]
}

pub fn default_tasks() -> DashMap<String, TaskConfig> {
    let tasks = DashMap::new();
    tasks.insert(
        "simple-task-example".to_string(),
        TaskConfig {
            meta: TaskMeta {
                id: "simple-task-example".to_string(),
                ..Default::default()
            },
            ..Default::default()
        },
    );
    tasks
}

pub async fn create_task_manager_with_participant_tags() -> TaskManager {
    let task_manager = TaskManager::default();
    *task_manager.participant_tags_config.write().await = ParticipantTagsConfig {
        participant_tags: default_participant_tags(),
    };
    task_manager
}

pub fn create_task_manager_with_default_tasks() -> TaskManager {
    let mut task_release_phases = HashMap::new();
    task_release_phases.insert("".to_string(), DateTime::<FixedOffset>::default());
    let event_config = EventConfig {
        task_release_phases,
        ..Default::default()
    };
    TaskManager {
        tasks: default_tasks(),
        event_config: RwLock::new(event_config),
        ..Default::default()
    }
}
