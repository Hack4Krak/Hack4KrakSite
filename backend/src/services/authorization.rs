use crate::entities::sea_orm_active_enums::UserRoles;
use crate::entities::{teams, user_participant_tags, users};
use crate::models::task_manager::participant_tags_config::ParticipantTag;
use crate::services::task_manager::TaskManager;
use crate::utils::error::Error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserIdentificationInfo {
    pub identification_code: Uuid,
    pub applied_tags: Vec<ParticipantTag>,
}

pub struct AuthorizationService;

impl AuthorizationService {
    pub fn assert_user_has_role(
        user: &users::Model,
        required_role: UserRoles,
    ) -> Result<(), Error> {
        if user.roles < required_role {
            return Err(Error::Forbidden { required_role });
        }

        Ok(())
    }

    pub async fn apply_tag(
        database: &DatabaseConnection,
        task_manager: &TaskManager,
        identification_code: Uuid,
        tag_id: &str,
    ) -> Result<(), Error> {
        let participant_tags_config = task_manager.participant_tags_config.read().await;
        let is_presence_verification_tag =
            participant_tags_config.is_presence_verification_tag(tag_id)?;

        let user = users::Entity::find()
            .filter(users::Column::IdentificationCode.eq(identification_code))
            .one(database)
            .await?
            .ok_or(Error::InvalidIdentificationCode)?;

        if user_participant_tags::Model::has_tag(database, user.id, tag_id).await? {
            return Err(Error::TagAlreadyApplied {
                tag_id: tag_id.to_string(),
            });
        }

        if is_presence_verification_tag && let Some(team_id) = user.team {
            teams::Model::confirm(database, team_id).await?;
        }

        user_participant_tags::Model::add_tag(database, user.id, tag_id).await?;

        Ok(())
    }
    pub async fn user_tags(
        database: &DatabaseConnection,
        task_manager: &TaskManager,
        user_id: Uuid,
    ) -> Result<Vec<ParticipantTag>, Error> {
        let tag_ids = user_participant_tags::Model::tag_ids_for_user(database, user_id).await?;

        let tags_config = task_manager.participant_tags_config.read().await;

        let applied_tags = tag_ids
            .into_iter()
            .map(|tag| {
                tags_config
                    .tag_by_id(&tag)
                    .cloned()
                    .ok_or(Error::InvalidTagId { tag_id: tag })
            })
            .collect::<Result<Vec<_>, _>>()?;

        Ok(applied_tags)
    }
}
