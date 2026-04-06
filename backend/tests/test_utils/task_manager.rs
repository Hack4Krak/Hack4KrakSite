use hack4krak_backend::models::task::{ParticipantTag, ParticipantTagsConfig};
use hack4krak_backend::services::task_manager::TaskManager;

pub fn default_participant_tags() -> Vec<ParticipantTag> {
    vec![
        ParticipantTag {
            id: "sponsor".to_string(),
            name: "Sponsor".to_string(),
            description: "Event sponsor".to_string(),
        },
        ParticipantTag {
            id: "volunteer".to_string(),
            name: "Volunteer".to_string(),
            description: "Event volunteer".to_string(),
        },
        ParticipantTag {
            id: "judge".to_string(),
            name: "Judge".to_string(),
            description: "Event judge".to_string(),
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
