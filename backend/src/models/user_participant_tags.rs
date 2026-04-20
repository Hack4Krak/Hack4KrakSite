use crate::entities::user_participant_tags;
use crate::utils::error::Error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter, Set};
use uuid::Uuid;

impl user_participant_tags::Model {
    pub async fn has_tag(
        database_connection: &DatabaseConnection,
        user_id: Uuid,
        tag_id: &str,
    ) -> Result<bool, Error> {
        let existing_tag = user_participant_tags::Entity::find()
            .filter(user_participant_tags::Column::UserId.eq(user_id))
            .filter(user_participant_tags::Column::TagId.eq(tag_id))
            .one(database_connection)
            .await?;

        Ok(existing_tag.is_some())
    }

    pub async fn add_tag(
        database_connection: &DatabaseConnection,
        user_id: Uuid,
        tag_id: &str,
    ) -> Result<(), Error> {
        user_participant_tags::Entity::insert(user_participant_tags::ActiveModel {
            user_id: Set(user_id),
            tag_id: Set(tag_id.to_string()),
        })
        .exec(database_connection)
        .await?;

        Ok(())
    }

    pub async fn tag_ids_for_user(
        database_connection: &DatabaseConnection,
        user_id: Uuid,
    ) -> Result<Vec<String>, Error> {
        let tags = user_participant_tags::Entity::find()
            .filter(user_participant_tags::Column::UserId.eq(user_id))
            .all(database_connection)
            .await?;

        Ok(tags.into_iter().map(|tag| tag.tag_id).collect())
    }
}
