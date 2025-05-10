use crate::entities::{external_team_invitation, teams, users};
use crate::models::task::RegistrationConfig;
use crate::utils::error::Error;
use migration::JoinType;
use rand::Rng;
use rand::distr::Alphanumeric;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, Set};
use sea_orm::{ColumnTrait, IntoActiveModel, TransactionTrait};
use sea_orm::{DbErr, QueryFilter, QuerySelect, RelationTrait};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TeamCodes {
    team_name: String,
    codes: Vec<String>,
}

impl external_team_invitation::Model {
    pub async fn create(
        database: &impl sea_orm::ConnectionTrait,
        team_id: Uuid,
        administration_code: Uuid,
    ) -> Result<String, Error> {
        let rng = rand::rng();
        let code: String = rng
            .sample_iter(&Alphanumeric)
            .filter(|c| c.is_ascii_alphanumeric()) // Ensures the characters are alphanumeric
            .take(6) // Limits to 6 characters
            .map(char::from)
            .collect();

        external_team_invitation::ActiveModel {
            id: Set(Uuid::new_v4()),
            team: Set(team_id),
            access_code: Set(code.clone()),
            administration_code: Set(administration_code),
        }
        .insert(database)
        .await?;

        Ok(code)
    }

    pub async fn accept_invitation(
        database: &DatabaseConnection,
        registration_config: &RegistrationConfig,
        access_code: String,
        user: users::Model,
    ) -> Result<(), Error> {
        let invitation = external_team_invitation::Entity::find()
            .filter(external_team_invitation::Column::AccessCode.eq(access_code))
            .one(database)
            .await?
            .ok_or(Error::InvalidEmailConfirmationCode)?;

        teams::Model::assert_correct_team_size(
            database,
            registration_config.max_team_size,
            &invitation.team,
        )
        .await?;

        let transaction = database.begin().await?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.team = Set(Some(invitation.team));
        active_user.update(&transaction).await?;

        invitation.into_active_model().delete(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn list(
        database: &DatabaseConnection,
        access_code: Uuid,
    ) -> Result<Vec<(String, String)>, DbErr> {
        external_team_invitation::Entity::find()
            .filter(external_team_invitation::Column::AdministrationCode.eq(access_code))
            .join(
                JoinType::InnerJoin,
                external_team_invitation::Relation::Teams.def(),
            )
            .select_only()
            .column(external_team_invitation::Column::AccessCode)
            .column(teams::Column::Name)
            .into_tuple::<(String, String)>()
            .all(database)
            .await
    }

    pub async fn grouped_codes(
        database: &DatabaseConnection,
        access_code: Uuid,
    ) -> Result<Vec<TeamCodes>, DbErr> {
        let results = external_team_invitation::Model::list(database, access_code).await?;

        let mut map: HashMap<String, Vec<String>> = HashMap::new();
        for (access_code, team_name) in results {
            map.entry(team_name).or_default().push(access_code);
        }

        let grouped: Vec<TeamCodes> = map
            .into_iter()
            .map(|(team_name, codes)| TeamCodes { team_name, codes })
            .collect();

        Ok(grouped)
    }
}
