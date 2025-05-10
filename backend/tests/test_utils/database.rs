use crate::test_utils;
use chrono::Utc;
use hack4krak_backend::entities::sea_orm_active_enums::{TeamStatus, UserRoles};
use hack4krak_backend::entities::{teams, users};
use sea_orm::{DatabaseConnection, EntityTrait};
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
            email: "example@gmail.com".to_string(),
            created_at: Utc::now().naive_utc(),
            team: None,
            is_leader: false,
            // Password is Dziengiel
            password: Some("$argon2id$v=19$m=19456,t=2,p=1$GuyDKoLJCF5tt+MDGJqRfA$8NZPkyNbR/IWuLg6tR7tn0RH/lJGahLYDODj23ajP3Y".to_string()),
            roles: UserRoles::Default,
            personal_info: None,
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
            confirmation_code: Default::default(),
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
}
