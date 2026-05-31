use crate::test_utils;
use chrono::Utc;
use hack4krak_backend::entities::sea_orm_active_enums::{TeamStatus, UserRoles};
use hack4krak_backend::entities::teams::Model;
use hack4krak_backend::entities::{
    email_verification_request, event_registration, flag_capture, team_invites, teams, users,
};
use sea_orm::{DatabaseConnection, EntityTrait, QueryOrder, Set};
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
            identification_code: uuid,
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

    pub async fn with_team_invite(&self, user_id: Uuid, team_id: Uuid) -> team_invites::Model {
        let invite_id = Uuid::new_v4();

        team_invites::Entity::insert(team_invites::ActiveModel {
            id: Set(invite_id),
            user: Set(user_id),
            team: Set(team_id),
        })
        .exec(&self.database)
        .await
        .unwrap();

        team_invites::Entity::find_by_id(invite_id)
            .one(&self.database)
            .await
            .unwrap()
            .unwrap()
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

    pub async fn with_flag_capture(&self, team: &Model, task: String) -> flag_capture::Model {
        flag_capture::Entity::insert(flag_capture::ActiveModel {
            team: Set(team.id),
            task: Set(task),
            submitted_at: Set(Utc::now().naive_utc()),
            ..Default::default()
        })
        .exec(&self.database)
        .await
        .unwrap();

        flag_capture::Entity::find()
            .order_by_desc(flag_capture::Column::Id)
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

    pub async fn with_event_registration(
        &self,
        updatable_model: event_registration::UpdatableModel,
    ) -> event_registration::Model {
        let id = Uuid::new_v4();

        let updated = updatable_model.update(event_registration::Model {
            id,
            user_id: id,
            full_name: "dziengiel".to_string(),
            school: "zerya".to_string(),
            birth_year: "2001".to_string(),
            phone: "+909090909090".to_string(),
            is_underage: false,
            emergency_contact_name: None,
            emergency_contact_phone: None,
            emergency_contact_email: None,
            food_preference: Default::default(),
            food_allergies: None,
            registered_at: Utc::now().naive_utc(),
        });

        event_registration::Entity::insert(updated)
            .exec(&self.database)
            .await
            .unwrap();

        event_registration::Entity::find_by_id(id)
            .one(&self.database)
            .await
            .unwrap()
            .unwrap()
    }
}
