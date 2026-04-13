use hack4krak_backend::models::task_manager::participant_tags_config::{
    ParticipantTag, ParticipantTagsConfig,
};
use hack4krak_backend::services::task_manager::TaskManager;

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

pub async fn create_task_manager_with_participant_tags(
    participant_tags: Vec<ParticipantTag>,
) -> TaskManager {
    let task_manager = TaskManager::default();
    *task_manager.participant_tags_config.write().await =
        ParticipantTagsConfig { participant_tags };
    task_manager
}

pub async fn create_default_test_task_manager() -> TaskManager {
    create_task_manager_with_participant_tags(default_participant_tags()).await
}
