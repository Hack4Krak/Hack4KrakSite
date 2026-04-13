use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct LabelsConfig {
    pub labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
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
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct TaskMeta {
    pub id: String,
    pub name: String,
    pub labels: Vec<String>,
    pub difficulty_estimate: String,
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
