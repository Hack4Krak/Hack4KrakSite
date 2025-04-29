use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct EventConfig {
    pub start_date: DateTime<FixedOffset>,
    pub end_date: DateTime<FixedOffset>,
    pub id: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct RegistrationConfig {
    pub start_date: DateTime<FixedOffset>,
    pub end_date: DateTime<FixedOffset>,
    pub max_team_size: u16,
    pub registration_mode: RegistrationMode,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RegistrationMode {
    Internal,
    External,
}

impl Default for RegistrationConfig {
    fn default() -> Self {
        RegistrationConfig {
            start_date: DateTime::default(),
            end_date: DateTime::default(),
            max_team_size: 5,
            registration_mode: RegistrationMode::Internal,
        }
    }
}

impl Default for EventConfig {
    fn default() -> Self {
        EventConfig {
            id: "tasks".to_string(),
            start_date: DateTime::default(),
            end_date: DateTime::default(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Default)]
pub struct TaskConfig {
    #[serde(flatten)]
    pub description: TaskDescription,
    #[serde(skip_serializing)]
    pub flag_hash: String,
    #[serde(default)]
    pub assets: Vec<TaskAsset>,
    pub story: Vec<TaskStory>,
    pub display: TaskDisplay,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct TaskDescription {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TaskStory {
    pub title: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct TaskDisplay {
    pub icon_coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TaskAsset {
    pub description: String,
    pub path: String,
}
