use crate::models::task::{EventConfig, RegistrationConfig, TaskConfig};
use crate::routes::task::TaskError;
use crate::services::env::EnvConfig;
use crate::utils::error::Error;
use actix_files::NamedFile;
use dashmap::DashMap;
use dashmap::mapref::multiple::RefMulti;
use dashmap::mapref::one::Ref;
use sha2::{Digest, Sha256};
use tokio::fs;
use tokio::sync::RwLock;

#[derive(Default)]
pub struct TaskManager {
    pub event_config: RwLock<EventConfig>,
    pub registration_config: RwLock<RegistrationConfig>,
    pub tasks: DashMap<String, TaskConfig>,
}

impl TaskManager {
    pub async fn refresh(&self) {
        self.tasks.clear();
        Self::load_tasks(&self.tasks).await;
    }

    async fn load_tasks(tasks: &DashMap<String, TaskConfig>) {
        let mut entries = fs::read_dir(&EnvConfig::get().tasks_base_path.join("tasks/"))
            .await
            .unwrap();

        while let Ok(Some(entry)) = entries.next_entry().await {
            if !entry.metadata().await.unwrap().is_dir() {
                continue;
            }
            let path = entry.path();
            let file_content = fs::read_to_string(path.join("config.yaml")).await.unwrap();

            if let Ok(task) = serde_yml::from_str::<TaskConfig>(&file_content) {
                tasks.insert(task.description.id.clone(), task);
            }
        }
    }

    async fn load_config<T: serde::de::DeserializeOwned>(path: &str) -> T {
        let path = EnvConfig::get().tasks_base_path.join(path);

        let file_content = fs::read_to_string(path).await.unwrap();
        serde_yml::from_str::<T>(&file_content).unwrap()
    }

    pub async fn load() -> Self {
        let tasks = DashMap::new();

        Self::load_tasks(&tasks).await;

        let event_config: EventConfig = Self::load_config("config/event.yaml").await;
        let registration_config: RegistrationConfig =
            Self::load_config("config/registration.yaml").await;

        Self {
            event_config: RwLock::new(event_config),
            registration_config: RwLock::new(registration_config),
            tasks,
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

    pub fn find_by_flag(&self, flag: &str) -> Option<RefMulti<String, TaskConfig>> {
        let mut hasher = Sha256::new();
        hasher.update(flag);
        let hashed_flag = format!("{:x}", hasher.finalize());

        self.tasks.iter().find(|task| task.flag_hash == hashed_flag)
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
