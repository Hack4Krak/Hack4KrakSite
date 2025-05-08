use crate::entities::email_verification_request;
use crate::models::user::UserInformation;
use crate::utils::error::Error;
use chrono::{Duration, Utc};
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::ops::Add;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "snake_case", tag = "name", content = "data")]
pub enum EmailVerificationAction {
    ConfirmEmailAddress { user_information: UserInformation },
    ResetPassword,
    RegisterTeam { organization: String },
}

impl EmailVerificationAction {
    pub fn get(&self) -> (String, Option<Value>) {
        let value = serde_json::to_value(self).unwrap();
        let name = value.get("name").unwrap().as_str().unwrap().to_string();
        let data = value.get("data").cloned();

        (name, data)
    }
}

impl email_verification_request::Model {
    pub async fn create(
        database: &DatabaseConnection,
        action: EmailVerificationAction,
        email: String,
        expiration_duration: Option<Duration>,
    ) -> Result<Uuid, Error> {
        let (action_type, action_data) = action.get();
        let confirmation_code = Uuid::new_v4();

        email_verification_request::Entity::insert(email_verification_request::ActiveModel {
            id: Set(confirmation_code),
            action_type: Set(action_type),
            additional_data: Set(action_data),
            email: Set(email),
            expiration_time: Set(
                expiration_duration.map(|duration| Utc::now().naive_utc().add(duration))
            ),
            created_at: Set(Utc::now().naive_utc()),
        })
        .exec(database)
        .await?;

        Ok(confirmation_code)
    }

    pub async fn find_and_verify(database: &DatabaseConnection, id: Uuid) -> Result<Self, Error> {
        let email_verification_request = email_verification_request::Entity::find_by_id(id)
            .one(database)
            .await?
            .ok_or(Error::InvalidEmailConfirmationCode)?;

        if let Some(expiration_time) = email_verification_request.expiration_time {
            if expiration_time < Utc::now().naive_utc() {
                let active_email_confirmation: email_verification_request::ActiveModel =
                    email_verification_request.into();
                active_email_confirmation.delete(database).await?;

                return Err(Error::EmailConfirmationCodeExpired);
            }
        }

        Ok(email_verification_request)
    }

    pub async fn delete(self, database: &DatabaseConnection) -> Result<(), Error> {
        let active_email_verification_request: email_verification_request::ActiveModel =
            self.into();
        active_email_verification_request.delete(database).await?;

        Ok(())
    }

    pub fn get_action(&self) -> Result<EmailVerificationAction, Error> {
        let name = &self.action_type.trim().to_string();
        let data = self.additional_data.clone().unwrap_or(Value::Null);

        let value = serde_json::json!({
            "name": name,
            "data": data
        });

        let action: EmailVerificationAction = serde_json::from_value(value)?;
        Ok(action)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const NAME: &str = "cat";

    #[test]
    fn test_stored_action_confirm_email() {
        let action = EmailVerificationAction::ConfirmEmailAddress {
            user_information: UserInformation {
                name: NAME.to_string(),
                ..Default::default()
            },
        };
        let (name, data) = action.get();
        assert_eq!(name, "confirm_email_address");
        assert!(data.unwrap().to_string().contains(NAME));
    }

    #[test]
    fn test_get_action_confirm_email() {
        let action = EmailVerificationAction::ConfirmEmailAddress {
            user_information: UserInformation {
                name: NAME.to_string(),
                ..Default::default()
            },
        };
        let (action_type, additional_data) = action.get();

        let model = email_verification_request::Model {
            action_type,
            additional_data: Some(additional_data.unwrap()),
            ..Default::default()
        };

        let restored = model.get_action().unwrap();

        match restored {
            EmailVerificationAction::ConfirmEmailAddress { user_information } => {
                assert_eq!(NAME, user_information.name);
            }
            _ => panic!("Unexpected variant"),
        }
    }
}
