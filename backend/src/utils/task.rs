use crate::routes::task::TaskError;
use crate::utils::env::Config;
use crate::utils::error::Error;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use tokio::fs;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskConfig {
    #[serde(flatten)]
    pub description: TaskDescription,
    pub story: Vec<TaskStory>,
    pub display: TaskDisplay,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct TaskDescription {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskStory {
    pub title: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct TaskDisplay {
    pub icon_coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, ToSchema, Clone)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

pub struct TaskManager {
    pub tasks: HashMap<String, TaskConfig>,
}

impl TaskManager {
    pub async fn load() -> Self {
        let mut tasks = HashMap::new();
        let mut entries = fs::read_dir(&Config::get().tasks_base_path).await.unwrap();

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

    pub fn get_task(&self, id: &str) -> Result<&TaskConfig, Error> {
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

        let icon_path = Config::get().tasks_base_path.join(id).join(path);

        if !icon_path.exists() || !icon_path.is_file() {
            return Err(TaskError::CouldNotLoadTaskIcon { id: id.to_string() }.into());
        }

        Ok(fs::read(icon_path).await?)
    }
}
