use serde::{Deserialize, Serialize};
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
