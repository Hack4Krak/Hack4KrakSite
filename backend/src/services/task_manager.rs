use crate::models::task::TaskConfig;
use crate::routes::task::TaskError;
use crate::services::env::EnvConfig;
use crate::utils::error::Error;
use dashmap::mapref::one::Ref;
use dashmap::DashMap;
use tokio::fs;

#[derive(Default)]
pub struct TaskManager {
    pub tasks: DashMap<String, TaskConfig>,
}

impl TaskManager {
    pub async fn load() -> Self {
        let tasks = DashMap::new();
        let mut entries = fs::read_dir(&EnvConfig::get().tasks_base_path)
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

        Self { tasks }
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

    pub async fn load_asset(&self, id: &str, path: &str) -> Result<Vec<u8>, Error> {
        self.get_task(id)?;

        let asset_path = EnvConfig::get().tasks_base_path.join(id).join(path);

        if !asset_path.exists() || !asset_path.is_file() {
            return Err(TaskError::CouldNotLoadTaskAsset { id: id.to_string() }.into());
        }

        Ok(fs::read(asset_path).await?)
    }
}
