use crate::entities::user_participant_tags;
use crate::entities::user_participant_tags::Model;
use crate::utils::error::Error;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, DatabaseBackend, DatabaseConnection, EntityTrait, Set,
    Statement,
};
use uuid::Uuid;

impl Model {
    pub fn has_tag(&self, tag_id: &str) -> bool {
        if self.tags.iter().any(|tag| tag == tag_id) {
            return true;
        }
        false
    }

    pub async fn add_tag(
        self,
        database_connection: &DatabaseConnection,
        tag_id: &str,
    ) -> Result<(), Error> {
        let mut new_tag_list = self.tags.clone();
        new_tag_list.push(tag_id.to_string());

        let new_tag = user_participant_tags::UpdatableModel {
            tags: Some(new_tag_list),
        };

        let active_model = new_tag.update(self);
        active_model.save(database_connection).await?;

        Ok(())
    }

    pub async fn get_or_create(
        database_connection: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Model, Error> {
        // Used in tests
        if database_connection.get_database_backend() == DatabaseBackend::Sqlite {
            let existing = database_connection
                .query_one(Statement::from_sql_and_values(
                    DatabaseBackend::Sqlite,
                    "SELECT tags FROM user_participant_tags WHERE user_id = ?",
                    vec![user_id.to_string().into()],
                ))
                .await?;

            if let Some(row) = existing {
                let tags: String = row.try_get("", "tags")?;
                return Ok(Model {
                    user_id,
                    tags: serde_json::from_str(&tags)?,
                });
            }

            database_connection
                .execute(Statement::from_sql_and_values(
                    DatabaseBackend::Sqlite,
                    "INSERT INTO user_participant_tags (user_id, tags) VALUES (?, ?)",
                    vec![user_id.to_string().into(), "[]".into()],
                ))
                .await?;

            return Ok(Model {
                user_id,
                tags: Vec::new(),
            });
        }

        let participant_tags = user_participant_tags::Entity::find_by_id(user_id)
            .one(database_connection)
            .await?;

        match participant_tags {
            Some(tags) => Ok(tags),
            None => {
                user_participant_tags::Entity::insert(user_participant_tags::ActiveModel {
                    user_id: Set(user_id),
                    tags: Set(Vec::new()),
                })
                .exec(database_connection)
                .await?;

                Ok(Model {
                    user_id,
                    tags: Vec::new(),
                })
            }
        }
    }
}
