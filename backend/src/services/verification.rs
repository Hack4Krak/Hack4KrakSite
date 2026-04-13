use crate::entities::{user_participant_tags, users};
use crate::models::task_manager::participant_tags_config::ParticipantTag;
use crate::services::emails::VerificationQrCode;
use crate::services::task_manager::TaskManager;
use crate::utils::app_state::AppState;
use crate::utils::email::Email;
use crate::utils::error::Error;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct VerifiedUserInfo {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub team_name: Option<String>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct UserVerificationInfo {
    pub verification_id: Uuid,
    pub applied_tags: Vec<ParticipantTag>,
}

pub struct VerificationService;

impl VerificationService {
    pub async fn send_verification_qr_email(
        app_state: &AppState,
        username: &str,
        email: &str,
        verification_id: Uuid,
    ) -> Result<(), Error> {
        Email::new(
            "verification",
            vec![email.to_string()],
            Box::new(VerificationQrCode {
                user: username.to_string(),
                verification_id: verification_id.to_string(),
            }),
        )
        .send(app_state.smtp_client.as_ref())
        .await?;

        Ok(())
    }

    pub async fn identify_user(
        database: &DatabaseConnection,
        verification_id: Uuid,
    ) -> Result<VerifiedUserInfo, Error> {
        let user = users::Entity::find()
            .filter(users::Column::VerificationId.eq(verification_id))
            .one(database)
            .await?
            .ok_or(Error::InvalidVerificationId)?;

        let team_name = user.get_team(database).await?.map(|team| team.name);

        Ok(VerifiedUserInfo {
            user_id: user.id,
            username: user.username,
            email: user.email,
            team_name,
        })
    }

    pub async fn apply_tag(
        database: &DatabaseConnection,
        task_manager: &TaskManager,
        verification_id: Uuid,
        tag_id: &str,
    ) -> Result<(), Error> {
        let participant_tags_config = task_manager.participant_tags_config.read().await;
        if !participant_tags_config.tag_exists(tag_id) {
            return Err(Error::InvalidTagId {
                tag_id: tag_id.to_string(),
            });
        }

        let user = users::Entity::find()
            .filter(users::Column::VerificationId.eq(verification_id))
            .one(database)
            .await?
            .ok_or(Error::InvalidVerificationId)?;

        let participant_tags_list =
            user_participant_tags::Model::get_or_create(database, user.id).await?;

        if participant_tags_list.has_tag(tag_id) {
            return Err(Error::TagAlreadyApplied {
                tag_id: tag_id.to_string(),
            });
        }

        participant_tags_list.add_tag(database, tag_id).await?;

        Ok(())
    }

    async fn user_tags(
        database: &DatabaseConnection,
        task_manager: &TaskManager,
        user_id: Uuid,
    ) -> Result<Vec<ParticipantTag>, Error> {
        let tags = user_participant_tags::Model::get_or_create(database, user_id).await?;

        let tags_config = task_manager.participant_tags_config.read().await;

        let applied_tags = tags
            .tags
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

    pub async fn user_verification_info(
        database: &DatabaseConnection,
        task_manager: &TaskManager,
        user: &users::Model,
    ) -> Result<UserVerificationInfo, Error> {
        let applied_tags = Self::user_tags(database, task_manager, user.id).await?;

        Ok(UserVerificationInfo {
            verification_id: user.verification_id,
            applied_tags,
        })
    }

    pub async fn reset_verification_id(app_state: &AppState, user_id: Uuid) -> Result<(), Error> {
        let user = users::Entity::find_by_id(user_id)
            .one(&app_state.database)
            .await?
            .ok_or(Error::UserNotFound)?;

        let username = user.username.clone();
        let user_email = user.email.clone();

        let new_uuid = Uuid::new_v4();

        users::Model::update(
            &app_state.database,
            user,
            users::UpdatableModel {
                verification_id: Some(new_uuid),
                ..Default::default()
            },
        )
        .await?;

        Self::send_verification_qr_email(app_state, &username, &user_email, new_uuid).await?;

        Ok(())
    }
}
