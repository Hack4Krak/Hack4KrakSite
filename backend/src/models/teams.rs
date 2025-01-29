use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, TransactionTrait};

use crate::models::entities::{teams, users};
use crate::routes::teams::create_team::CreateTeamModel;
use crate::routes::teams::TeamError::{AlreadyExists, UserAlreadyBelongsToTeam};
use crate::utils::error::Error;
use crate::utils::jwt::Claims;

impl teams::Model {
    pub async fn create_team(
        database: &DatabaseConnection,
        create_team_json: CreateTeamModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find_by_id(&claim_data.email)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if let Some(team_name) = user.team_name {
            return Err(Error::Team(UserAlreadyBelongsToTeam { team_name }));
        }

        if teams::Entity::find()
            .filter(teams::Column::Name.eq(&create_team_json.team_name))
            .one(database)
            .await?
            .is_some()
        {
            return Err(Error::Team(AlreadyExists));
        }

        let transaction = database.begin().await?;

        let team_name = create_team_json.team_name;

        teams::Entity::insert(teams::ActiveModel {
            name: Set(team_name.clone()),
            leader_name: Set(user.username.clone()),
            created_at: Set(Utc::now().naive_utc()),
        })
        .exec(&transaction)
        .await?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.team_name = Set(Some(team_name));
        active_user.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}
