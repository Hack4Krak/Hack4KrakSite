use crate::entities::announcement;
use crate::utils::error::Error;
use chrono::Utc;
use sea_orm::{
    ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, QueryOrder, QuerySelect, Set,
};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, PartialEq)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum AnnouncementAction {
    Normal {
        text: String,
    },
    TaskStatusUpdate {
        task_id: String,
        status: TaskStatus,
        comment: Option<String>,
    },
}

#[derive(Serialize, Deserialize, Debug, Clone, ToSchema, Eq, PartialEq)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Up,
    Broken,
    Down,
}

impl AnnouncementAction {
    pub fn get(&self) -> (String, Option<Value>) {
        let value = serde_json::to_value(self).unwrap();
        let name = value.get("type").unwrap().as_str().unwrap().to_string();

        let mut data = value.clone();
        if let Some(obj) = data.as_object_mut() {
            obj.remove("type");
        }

        (name, Some(data))
    }
}

impl announcement::Model {
    pub async fn create(
        database: &DatabaseConnection,
        action: &AnnouncementAction,
    ) -> Result<(), Error> {
        let (action_type, action_data) = action.get();

        announcement::Entity::insert(announcement::ActiveModel {
            id: Set(Uuid::new_v4()),
            action_type: Set(action_type),
            additional_data: Set(action_data),
            created_at: Set(Utc::now().naive_utc()),
        })
        .exec(database)
        .await?;

        Ok(())
    }

    pub fn action(&self) -> Result<AnnouncementAction, Error> {
        let mut value = self.additional_data.clone().unwrap_or_default();
        if let Some(object) = value.as_object_mut() {
            object.insert("type".to_string(), Value::String(self.action_type.clone()));
        } else {
            value = serde_json::json!({
                "type": self.action_type
            });
        }

        let action: AnnouncementAction = serde_json::from_value(value)?;
        Ok(action)
    }

    pub async fn find_paginated(
        database: &DatabaseConnection,
        action_type: Option<&str>,
        limit: u64,
        offset: u64,
    ) -> Result<Vec<Self>, Error> {
        let mut query = announcement::Entity::find();
        if let Some(action_type) = action_type {
            query = query.filter(announcement::Column::ActionType.eq(action_type));
        }

        let models = query
            .order_by_desc(announcement::Column::CreatedAt)
            .limit(limit)
            .offset(offset)
            .all(database)
            .await?;

        Ok(models)
    }

    pub async fn latest_task_updates(
        database: &DatabaseConnection,
    ) -> Result<HashMap<String, TaskStatus>, Error> {
        let models = announcement::Entity::find()
            .filter(announcement::Column::ActionType.eq("task_status_update"))
            .order_by_desc(announcement::Column::CreatedAt)
            .all(database)
            .await?;

        let mut statuses = HashMap::new();
        for model in models {
            let parsed_model = model.action()?;
            if let AnnouncementAction::TaskStatusUpdate {
                task_id, status, ..
            } = parsed_model
            {
                statuses.entry(task_id).or_insert(status);
            }
        }

        Ok(statuses)
    }
}
