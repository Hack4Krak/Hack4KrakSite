use crate::entities::announcement;
use crate::models::announcement::{AnnouncementAction, TaskStatus};
use crate::models::task_manager::event_config::EventConfig;
use crate::models::task_manager::participant_tags_config::ParticipantTagsConfig;
use crate::models::task_manager::registration_config::RegistrationConfig;
use crate::models::task_manager::task_config::{LabelsConfig, TaskConfig, TaskDisplay, TaskMeta};
use crate::routes::task::TaskError;
use crate::services::env::EnvConfig;
use crate::utils::error::Error;
use actix_files::NamedFile;
use dashmap::DashMap;
use dashmap::mapref::multiple::RefMulti;
use dashmap::mapref::one::Ref;
use sea_orm::DatabaseConnection;
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::HashMap;
use tokio::fs;
use tokio::sync::RwLock;
use tracing::error;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct SimpleTask {
    #[serde(flatten)]
    pub description: TaskMeta,
    pub display: TaskDisplay,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct TaskWithStatus {
    #[serde(flatten)]
    pub task: SimpleTask,
    pub status: TaskStatus,
}

impl TaskWithStatus {
    pub fn merge_tasks_with_statuses(
        tasks: Vec<SimpleTask>,
        statuses: HashMap<String, TaskStatus>,
    ) -> Vec<TaskWithStatus> {
        tasks
            .into_iter()
            .map(|task| {
                let status = statuses
                    .get(&task.description.id)
                    .cloned()
                    .unwrap_or(TaskStatus::Up);
                TaskWithStatus { task, status }
            })
            .collect()
    }
}

#[derive(Default)]
pub struct TaskManager {
    pub event_config: RwLock<EventConfig>,
    pub registration_config: RwLock<RegistrationConfig>,
    pub labels_config: RwLock<LabelsConfig>,
    pub participant_tags_config: RwLock<ParticipantTagsConfig>,
    pub tasks: DashMap<String, TaskConfig>,
    pub task_status_cache: RwLock<Option<HashMap<String, TaskStatus>>>,
}

impl LabelsConfig {
    pub async fn load_label(&self, id: &str) -> Result<NamedFile, Error> {
        // todo: check if id is in self labels

        let asset_path = EnvConfig::get()
            .tasks_base_path
            .join("config/assets/labels")
            .join(format!("{id}.png"));

        if !asset_path.exists() || !asset_path.is_file() {
            return Err(TaskError::CouldNotLoadTaskAsset { id: id.to_string() }.into());
        }

        let named_file = NamedFile::open(asset_path)?;

        Ok(named_file)
    }
}

impl TaskManager {
    pub async fn refresh(&self) {
        // todo: refreshconfigs
        self.tasks.clear();
        Self::load_tasks(&self.tasks).await;
        self.invalidate_task_status_cache().await;
    }

    pub fn tasks(&self) -> Vec<SimpleTask> {
        let tasks: Vec<SimpleTask> = self
            .tasks
            .iter()
            .map(|task| SimpleTask {
                description: task.meta.clone(),
                display: task.display.clone(),
            })
            .collect();

        tasks
    }

    pub fn tasks_sorted(&self) -> Vec<SimpleTask> {
        let mut tasks = self.tasks();

        tasks.sort_by(|a, b| a.description.id.cmp(&b.description.id));

        tasks
    }

    async fn load_tasks(tasks: &DashMap<String, TaskConfig>) {
        let tasks_dir = EnvConfig::get().tasks_base_path.join("tasks/");
        let mut entries = fs::read_dir(&tasks_dir)
            .await
            .unwrap_or_else(|err| panic!("Failed to read tasks directory {tasks_dir:?}: {err}"));

        loop {
            let entry = match entries.next_entry().await {
                Ok(Some(e)) => e,
                Ok(None) => break,
                Err(err) => {
                    error!("Failed to read directory entry in {tasks_dir:?}: {err}");
                    break;
                }
            };

            let path = entry.path();

            let metadata = match entry.metadata().await {
                Ok(m) => m,
                Err(err) => {
                    error!("Failed to read metadata for {path:?}: {err}");
                    continue;
                }
            };

            if !metadata.is_dir() {
                continue;
            }

            let config_path = path.join("config.yaml");
            let file_content = match fs::read_to_string(&config_path).await {
                Ok(content) => content,
                Err(err) => {
                    error!("Failed to read task config at {config_path:?}: {err}");
                    continue;
                }
            };

            match serde_norway::from_str::<TaskConfig>(&file_content) {
                Ok(task) => {
                    tasks.insert(task.meta.id.clone(), task);
                }
                Err(err) => {
                    error!("Failed to parse task config at {config_path:?}: {err}");
                }
            }
        }
    }

    async fn load_config<T: serde::de::DeserializeOwned>(path: &str) -> T {
        let path = EnvConfig::get().tasks_base_path.join(path);

        let file_content = fs::read_to_string(path).await.unwrap();
        serde_norway::from_str::<T>(&file_content).unwrap()
    }

    pub async fn load() -> Self {
        let tasks = DashMap::new();

        Self::load_tasks(&tasks).await;

        let event_config: EventConfig = Self::load_config("config/event.yaml").await;
        let registration_config: RegistrationConfig =
            Self::load_config("config/registration.yaml").await;

        let labels_config: LabelsConfig = Self::load_config("config/labels.yaml").await;

        let participant_tags_config: ParticipantTagsConfig =
            Self::load_config("config/participant-tags.yaml").await;

        Self {
            event_config: RwLock::new(event_config),
            labels_config: RwLock::new(labels_config),
            participant_tags_config: RwLock::new(participant_tags_config),
            registration_config: RwLock::new(registration_config),
            tasks,
            task_status_cache: RwLock::new(None),
        }
    }

    pub fn get_task(&self, id: &str) -> Result<Ref<'_, String, TaskConfig>, Error> {
        if !id
            .chars()
            .all(|char| char.is_ascii_alphanumeric() || char == '-' || char == '_')
        {
            return Err(TaskError::InvalidTaskId.into());
        }

        self.tasks
            .get(id)
            .ok_or(TaskError::MissingTask { id: id.to_string() }.into())
    }

    pub async fn load_asset(&self, id: &str, path: &str) -> Result<NamedFile, Error> {
        self.get_task(id)?;

        let asset_path = EnvConfig::get()
            .tasks_base_path
            .join("tasks/")
            .join(id)
            .join(path);

        if !asset_path.exists() || !asset_path.is_file() {
            return Err(TaskError::CouldNotLoadTaskAsset { id: id.to_string() }.into());
        }

        let named_file = NamedFile::open(asset_path)?;

        Ok(named_file)
    }

    pub fn find_by_flag(&self, flag: &str) -> Option<RefMulti<'_, String, TaskConfig>> {
        let hashed_flag = Sha256::digest(flag);
        let hashed_flag_hex = hashed_flag
            .iter()
            .map(|byte| format!("{:02x}", byte))
            .collect::<String>();

        self.tasks
            .iter()
            .find(|task| task.flag_hash == hashed_flag_hex)
    }

    async fn invalidate_task_status_cache(&self) {
        *self.task_status_cache.write().await = None;
    }

    pub async fn update_task_status(
        &self,
        database: &DatabaseConnection,
        action: &AnnouncementAction,
    ) -> Result<announcement::Model, Error> {
        if let AnnouncementAction::TaskStatusUpdate { task_id, .. } = action {
            self.get_task(task_id)?;
        }
        self.invalidate_task_status_cache().await;

        announcement::Model::create(database, action).await
    }

    async fn tasks_with_statuses(
        &self,
        tasks_list: Vec<SimpleTask>,
        database: &DatabaseConnection,
    ) -> Result<Vec<TaskWithStatus>, Error> {
        {
            let cache = self.task_status_cache.read().await;
            if let Some(cached) = cache.as_ref() {
                return Ok(TaskWithStatus::merge_tasks_with_statuses(
                    tasks_list,
                    cached.clone(),
                ));
            }
        }

        let statuses = announcement::Model::latest_task_updates(database).await?;

        let mut cache = self.task_status_cache.write().await;
        *cache = Some(statuses.clone());

        Ok(TaskWithStatus::merge_tasks_with_statuses(
            tasks_list, statuses,
        ))
    }

    pub async fn list_tasks(
        &self,
        database: &DatabaseConnection,
    ) -> Result<Vec<TaskWithStatus>, Error> {
        let tasks = self.tasks_sorted();
        self.tasks_with_statuses(tasks, database).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::default::Default;

    const FLAG_HASH: &str = "1912766d6ba0e50e8b1bacfb51207e83b95b7ac0cd8ce15307cdf4965e7e3f6c";

    #[test]
    fn find_existing_flag() {
        let task_manager = TaskManager::default();

        task_manager.tasks.insert(
            "test-task".to_string(),
            TaskConfig {
                flag_hash: FLAG_HASH.to_string(),
                ..Default::default()
            },
        );

        let flag = task_manager.find_by_flag("skibidi");
        assert!(flag.is_some());
        assert_eq!(flag.unwrap().flag_hash, FLAG_HASH);
    }

    #[test]
    fn find_non_existing_flag() {
        let task_manager = TaskManager::default();

        task_manager.tasks.insert(
            "test-task".to_string(),
            TaskConfig {
                flag_hash: FLAG_HASH.to_string(),
                ..Default::default()
            },
        );

        let flag = task_manager.find_by_flag("asdsdads");
        assert!(flag.is_none());

        let flag = task_manager.find_by_flag("..asd3#1s--.?");
        assert!(flag.is_none());
    }
}
