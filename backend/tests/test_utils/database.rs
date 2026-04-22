use crate::test_utils;
use chrono::Utc;
use hack4krak_backend::entities::sea_orm_active_enums::{TeamStatus, UserRoles};
use hack4krak_backend::entities::{email_verification_request, teams, users};
use sea_orm::{DatabaseConnection, EntityTrait};
use serde_json::json;
use uuid::Uuid;

#[derive(Default)]
pub struct TestDatabase {
    pub database: DatabaseConnection,
}

impl TestDatabase {
    pub async fn new() -> Self {
        TestDatabase {
            database: test_utils::setup_database_with_schema().await,
        }
    }

    pub async fn with_default_user(&self) -> users::Model {
        self.with_user(Default::default()).await
    }

    pub async fn with_user(&self, updatable_model: users::UpdatableModel) -> users::Model {
        let uuid = Uuid::new_v4();

        let updated = updatable_model.update(users::Model {
            id: uuid,
            username: "test_user".to_string(),
            first_name: None,
            email: "example@gmail.com".to_string(),
            created_at: Utc::now().naive_utc(),
            team: None,
            is_leader: false,
            // Password is Dziengiel
            password: Some("$argon2id$v=19$m=19456,t=2,p=1$GuyDKoLJCF5tt+MDGJqRfA$8NZPkyNbR/IWuLg6tR7tn0RH/lJGahLYDODj23ajP3Y".to_string()),
            roles: UserRoles::Default,
            onboarding: None,
            verification_id: uuid,
        });

        users::Entity::insert(updated)
            .exec(&self.database)
            .await
            .unwrap();

        users::Entity::find_by_id(uuid)
            .one(&self.database)
            .await
            .unwrap()
            .unwrap()
    }

    pub async fn with_default_team(&self) -> teams::Model {
        self.with_team(Default::default()).await
    }

    pub async fn with_team(&self, updatable_model: teams::UpdatableModel) -> teams::Model {
        let team_uuid = Uuid::new_v4();

        let updated = updatable_model.update(teams::Model {
            id: team_uuid,
            name: "Dziengiel".to_string(),
            created_at: Utc::now().naive_utc(),
            color: "#000000".to_string(),
            organization: Some("Hack4Krak".to_string()),
            status: TeamStatus::Absent,
        });

        teams::Entity::insert(updated)
            .exec(&self.database)
            .await
            .unwrap();

        teams::Entity::find_by_id(team_uuid)
            .one(&self.database)
            .await
            .unwrap()
            .unwrap()
    }

    pub async fn with_email_verification_request(
        &self,
        updatable_model: email_verification_request::UpdatableModel,
    ) -> email_verification_request::Model {
        let confirmation_code = Uuid::new_v4();
        let updated = updatable_model.update(email_verification_request::Model {
            id: confirmation_code,
            email: "".to_string(),
            action_type: "confirm_email_address".to_string(),
            additional_data: Some(json![{
            "user_information": {
                "name": "test_user",
                "first_name": "dziengiel",
                "email": "example@gmail.com",
                "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$nTzWdmrtGEOnwCocrg76xg$yv16FfDT5+meKwPmSiV+MF9kP8Man6bXZs+BloFTKIk".to_string(),
            }
            }]),
            expiration_time: Some(Utc::now().naive_utc() + chrono::Duration::minutes(30)),
            created_at: Utc::now().naive_utc(),
        });

        email_verification_request::Entity::insert(updated)
            .exec(&self.database)
            .await
            .unwrap();

        email_verification_request::Entity::find_by_id(confirmation_code)
            .one(&self.database)
            .await
            .unwrap()
            .unwrap()
    }
}
