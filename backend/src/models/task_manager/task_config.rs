use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct LabelsConfig {
    pub labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Label {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Default)]
pub struct TaskConfig {
    #[serde(flatten)]
    pub meta: TaskMeta,
    #[serde(skip_serializing)]
    pub flag_hash: String,
    #[serde(default)]
    pub assets: Vec<TaskAsset>,
    pub display: TaskDisplay,
    pub task_release_phase: String,
}

#[derive(
    Serialize, Deserialize, ToSchema, Debug, Clone, Default, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct TaskMeta {
    pub id: String,
    pub name: String,
    pub labels: Vec<String>,
    pub difficulty_estimate: String,
    #[serde(default)]
    pub authors: Vec<TaskAuthor>,
}

#[derive(
    Serialize, Deserialize, ToSchema, Debug, Clone, Default, Eq, Ord, PartialEq, PartialOrd,
)]
pub struct TaskAuthor {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct TaskDisplay {
    pub icon_coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct Coordinates {
    #[serde(rename(deserialize = "x"))]
    pub lng: f32,
    #[serde(rename(deserialize = "y"))]
    pub lat: f32,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TaskAsset {
    pub description: String,
    pub path: String,
}
