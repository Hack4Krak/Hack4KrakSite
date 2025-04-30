use crate::entities::email_confirmation;
use crate::entities::email_confirmation::ActiveModel;
use crate::models::user::UserInformation;
use crate::routes::auth::AuthError::{ConfirmationCodeExpired, InvalidConfirmationCode};
use crate::utils::error::Error;
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter};

impl email_confirmation::Model {
    pub async fn create_with_userinfo(
        database: &DatabaseConnection,
        user_info: UserInformation,
    ) -> Result<String, Error> {
        let confirmation_code = uuid::Uuid::new_v4().to_string();

        let email_confirmation_active_model = ActiveModel {
            code: Set(confirmation_code.clone()),
            email: Set(user_info.email.clone()),
            expiration_date: Set(Utc::now().naive_utc() + chrono::Duration::days(1)),
            user_info: Set(serde_json::to_value(user_info)?),
        };

        email_confirmation_active_model.insert(database).await?;

        Ok(confirmation_code)
    }

    pub async fn get_user_info(
        database: &DatabaseConnection,
        confirmation_code: String,
    ) -> Result<UserInformation, Error> {
        let email_confirmation_data = email_confirmation::Entity::find_by_id(confirmation_code)
            .one(database)
            .await?
            .ok_or(Error::Auth(InvalidConfirmationCode))?;

        if email_confirmation_data.expiration_date < Utc::now().naive_utc() {
            Self::remove_expired_and_confirmed(database, None).await?;
            return Err(Error::Auth(ConfirmationCodeExpired));
        }

        let user_info: UserInformation = serde_json::from_value(email_confirmation_data.user_info)?;

        Ok(user_info)
    }

    pub async fn remove_expired_and_confirmed(
        database: &DatabaseConnection,
        confirmed_code: Option<String>,
    ) -> Result<(), Error> {
        let expired_email_confirmations = email_confirmation::Entity::find()
            .filter(email_confirmation::Column::ExpirationDate.lte(Utc::now().naive_utc()))
            .all(database)
            .await?;

        for email_confirmation in expired_email_confirmations {
            let active_email_confirmation: ActiveModel = email_confirmation.into();
            email_confirmation::Entity::delete(active_email_confirmation)
                .exec(database)
                .await?;
        }

        if let Some(confirmed_code) = confirmed_code {
            let confirmed_email_confirmation =
                email_confirmation::Entity::find_by_id(confirmed_code)
                    .one(database)
                    .await?;

            if let Some(email_confirmation) = confirmed_email_confirmation {
                let active_email_confirmation: ActiveModel = email_confirmation.into();
                email_confirmation::Entity::delete(active_email_confirmation)
                    .exec(database)
                    .await?;
            }
        }

        Ok(())
    }
}
