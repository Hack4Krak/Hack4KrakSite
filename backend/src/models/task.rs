use chrono::{DateTime, FixedOffset};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "kebab-case")]
pub struct EventConfig {
    pub start_date: DateTime<FixedOffset>,
    pub end_date: DateTime<FixedOffset>,
    pub max_team_size: u16,
}

impl Default for EventConfig {
    fn default() -> Self {
        EventConfig {
            max_team_size: 5,
            start_date: DateTime::default(),
            end_date: DateTime::default(),
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Default)]
pub struct TaskConfig {
    #[serde(flatten)]
    pub description: TaskDescription,
    #[serde(skip_serializing)]
    pub flag_hash: String,
    pub assets: Vec<TaskAsset>,
    pub story: Vec<TaskStory>,
    pub display: TaskDisplay,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct TaskDescription {
    pub id: String,
    pub name: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskStory {
    pub title: String,
    pub message: String,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct TaskDisplay {
    pub icon_coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, ToSchema, Clone, Default)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TaskAsset {
    pub description: String,
    pub path: String,
}
