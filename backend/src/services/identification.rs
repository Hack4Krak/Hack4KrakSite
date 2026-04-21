use crate::entities::users;
use crate::services::authorization::{AuthorizationService, UserIdentificationInfo};
use crate::services::emails::IdentificationQrCode;
use crate::services::task_manager::TaskManager;
use crate::utils::app_state::AppState;
use crate::utils::email::{Email, EmailAttachment};
use crate::utils::error::Error;
use crate::utils::qr_code::generate_qr_png;
use lettre::message::header;
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct IdentifiedUserInfo {
    pub user_id: Uuid,
    pub username: String,
    pub email: String,
    pub team_name: Option<String>,
}

pub struct IdentificationService;

impl IdentificationService {
    pub async fn send_identification_code_email(
        app_state: &AppState,
        username: &str,
        email: &str,
        identification_code: Uuid,
    ) -> Result<(), Error> {
        let identification_code_string = identification_code.to_string();
        let qr_png = generate_qr_png(&identification_code_string)?;

        Email::new(
            "auth",
            vec![email.to_string()],
            Box::new(IdentificationQrCode {
                user: username.to_string(),
                identification_code: identification_code_string,
            }),
        )
        .with_attachment(EmailAttachment::new(
            "identification-qr.png",
            header::ContentType::parse("image/png")
                .map_err(|err| Error::FailedToGenerateQrCode(err.to_string()))?,
            qr_png,
        ))
        .send(app_state.smtp_client.as_ref())
        .await?;

        Ok(())
    }

    pub async fn identify_user(
        database: &DatabaseConnection,
        identification_code: Uuid,
    ) -> Result<IdentifiedUserInfo, Error> {
        let user = users::Entity::find()
            .filter(users::Column::IdentificationCode.eq(identification_code))
            .one(database)
            .await?
            .ok_or(Error::InvalidIdentificationCode)?;

        let team_name = user.get_team(database).await?.map(|team| team.name);

        Ok(IdentifiedUserInfo {
            user_id: user.id,
            username: user.username,
            email: user.email,
            team_name,
        })
    }

    pub async fn user_identification_info(
        database: &DatabaseConnection,
        task_manager: &TaskManager,
        user: &users::Model,
    ) -> Result<UserIdentificationInfo, Error> {
        let applied_tags = AuthorizationService::user_tags(database, task_manager, user.id).await?;

        Ok(UserIdentificationInfo {
            identification_code: user.identification_code,
            applied_tags,
        })
    }

    pub async fn reset_identification_code(
        app_state: &AppState,
        user_id: Uuid,
    ) -> Result<(), Error> {
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
                identification_code: Some(new_uuid),
                ..Default::default()
            },
        )
        .await?;

        Self::send_identification_code_email(app_state, &username, &user_email, new_uuid).await?;

        Ok(())
    }
}
