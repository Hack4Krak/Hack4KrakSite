use crate::entities::sea_orm_active_enums::TeamStatus;
use crate::entities::teams::ActiveModel;
use crate::entities::{external_team_invitation, flag_capture, teams, users};
use crate::models::task::RegistrationConfig;
use crate::routes::admin::UpdateTeamModel;
use crate::routes::teams::TeamError;
use crate::routes::teams::TeamError::*;
use crate::utils::error::Error;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use chrono::Utc;
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTime;
use sea_orm::{ActiveModelTrait, ConnectionTrait, ModelTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::future;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TeamWithMembers {
    pub id: Uuid,
    pub team_name: String,
    pub created_at: DateTime,
    pub members: Vec<(Uuid, String)>,
    pub confirmation_code: Option<Uuid>,
    pub status: TeamStatus,
}

impl teams::Model {
    pub async fn find_by_name(
        database: &impl ConnectionTrait,
        name: &str,
    ) -> Result<Option<Self>, Error> {
        let team = teams::Entity::find()
            .filter(teams::Column::Name.eq(name))
            .one(database)
            .await?;

        Ok(team)
    }

    pub async fn assert_team_size_before_adding_user(
        database: &DatabaseConnection,
        registration_config: &RegistrationConfig,
        id: &Uuid,
    ) -> Result<(), Error> {
        let members_count = users::Entity::find()
            .filter(users::Column::Team.eq(*id))
            .count(database)
            .await?;

        registration_config.assert_team_size(members_count as u16 + 1)
    }

    pub async fn try_create(
        transaction: &impl ConnectionTrait,
        team_name: String,
    ) -> Result<Uuid, Error> {
        if teams::Model::find_by_name(transaction, &team_name)
            .await?
            .is_some()
        {
            return Err(AlreadyExists { team_name }.into());
        }

        let uuid = Uuid::new_v4();
        teams::Entity::insert(teams::ActiveModel {
            id: Set(uuid),
            name: Set(team_name),
            created_at: Set(Utc::now().naive_utc()),
            status: Set(TeamStatus::Absent),
            ..Default::default()
        })
        .exec(transaction)
        .await?;

        Ok(uuid)
    }

    pub async fn create(
        database: &DatabaseConnection,
        team_name: String,
        user: users::Model,
    ) -> Result<(), Error> {
        let transaction = database.begin().await?;

        let uuid = Self::try_create(&transaction, team_name).await?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.team = Set(Some(uuid));
        active_user.is_leader = Set(true);
        active_user.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn create_as_external(
        database: &DatabaseConnection,
        registration_config: &RegistrationConfig,
        team_name: String,
        number_of_members: u16,
        administration_code: Uuid,
    ) -> Result<Vec<String>, Error> {
        registration_config.assert_team_size(number_of_members)?;

        let transaction = database.begin().await?;
        let team_id = Self::try_create(&transaction, team_name).await?;

        let mut invitations = Vec::new();
        for _ in 0..number_of_members {
            let invitation_code =
                external_team_invitation::Model::create(&transaction, team_id, administration_code)
                    .await?;

            invitations.push(invitation_code);
        }

        transaction.commit().await?;

        Ok(invitations)
    }

    pub async fn kick_user(
        database: &DatabaseConnection,
        user_to_remove: users::Model,
        user: users::Model,
    ) -> Result<(), Error> {
        if user_to_remove.id == user.id {
            return Err(Error::Team(UserCantRemoveYourself));
        }

        if user_to_remove.team != user.team && user_to_remove.team.is_some() {
            return Err(Error::Team(UserDoesntBelongToYourTeam));
        }

        if user_to_remove.is_leader {
            return Err(Error::Team(UserCantRemoveTeamLeader));
        }

        Self::remove_user(database, user_to_remove).await
    }

    pub async fn remove_user(
        database: &DatabaseConnection,
        user: users::Model,
    ) -> Result<(), Error> {
        let mut active_user: users::ActiveModel = user.into();
        active_user.team = Set(None);
        active_user.update(database).await?;

        Ok(())
    }

    pub async fn rename(
        database: &DatabaseConnection,
        new_team_name: String,
        team: teams::Model,
    ) -> Result<(), Error> {
        let mut active_team: ActiveModel = team.into();
        active_team.name = Set(new_team_name);
        active_team.update(database).await?;

        Ok(())
    }

    pub async fn change_leader(
        database: &DatabaseConnection,
        new_leader: users::Model,
        user: users::Model,
    ) -> Result<(), Error> {
        if new_leader.team != user.team {
            return Err(Error::Team(UserDoesntBelongToYourTeam));
        }

        let transaction = database.begin().await?;

        let mut active_user: users::ActiveModel = new_leader.into();
        active_user.is_leader = Set(true);
        active_user.update(&transaction).await?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.is_leader = Set(false);
        active_user.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn delete(
        database: &DatabaseConnection,
        user: users::Model,
        team: teams::Model,
    ) -> Result<(), Error> {
        let transaction = database.begin().await?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.is_leader = Set(false);
        active_user.update(&transaction).await?;

        team.delete(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn leader(
        database: &DatabaseConnection,
        team_id: Uuid,
    ) -> Result<users::Model, Error> {
        let leader = users::Entity::find()
            .filter(
                users::Column::Team
                    .eq(team_id)
                    .and(users::Column::IsLeader.eq(true)),
            )
            .one(database)
            .await?
            .ok_or(Error::Team(TeamLeaderNotFound))?;

        Ok(leader)
    }

    pub async fn list(database: &DatabaseConnection) -> Result<Vec<TeamWithMembers>, Error> {
        let teams = teams::Entity::find()
            .find_with_related(users::Entity)
            .all(database)
            .await?;

        let teams_with_members = teams
            .into_iter()
            .map(|(team, users)| {
                let members: Vec<(Uuid, String)> = users
                    .into_iter()
                    .map(|user| (user.id, user.username))
                    .collect();
                TeamWithMembers {
                    id: team.id,
                    team_name: team.name,
                    created_at: team.created_at,
                    members,
                    confirmation_code: team.confirmation_code,
                    status: team.status,
                }
            })
            .collect::<Vec<TeamWithMembers>>();

        Ok(teams_with_members)
    }

    pub async fn update(
        database: &DatabaseConnection,
        id: Uuid,
        update_team_json: UpdateTeamModel,
    ) -> Result<(), Error> {
        let team = teams::Entity::find_by_id(id)
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;

        if let Some(team_name) = update_team_json.team_name {
            if teams::Model::find_by_name(database, &team_name)
                .await?
                .is_some()
            {
                return Err(TeamError::AlreadyExists {
                    team_name: team.name,
                }
                .into());
            }
            let mut active_team: ActiveModel = team.clone().into();
            active_team.name = Set(team_name);
            active_team.update(database).await?;
        }

        if let Some(leader) = update_team_json.leader {
            let new_leader = users::Entity::find_by_id(leader)
                .one(database)
                .await?
                .ok_or(Error::UserNotFound)?;

            let leader = Self::leader(database, id).await?;

            Self::change_leader(database, new_leader, leader).await?;
        }

        if let Some(status) = update_team_json.status {
            let mut active_team: ActiveModel = team.into();
            active_team.status = Set(status);
            active_team.update(database).await?;
        }

        Ok(())
    }

    pub async fn delete_as_admin(database: &DatabaseConnection, id: Uuid) -> Result<(), Error> {
        let user = teams::Model::leader(database, id).await?;

        let transaction = database.begin().await?;

        let any_rows_affected = teams::Entity::delete_by_id(id)
            .exec(&transaction)
            .await?
            .rows_affected
            != 0;

        if !any_rows_affected {
            return Err(Error::Team(TeamNotFound));
        }

        let mut active_user: users::ActiveModel = user.into();
        active_user.is_leader = Set(false);
        active_user.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn confirm(
        database: &DatabaseConnection,
        confirmation_code: Uuid,
    ) -> Result<(), Error> {
        let team = teams::Entity::find()
            .filter(teams::Column::ConfirmationCode.eq(confirmation_code))
            .one(database)
            .await?
            .ok_or(Error::InvalidEmailConfirmationCode)?;

        let mut active_team: ActiveModel = team.clone().into();
        active_team.status = Set(TeamStatus::Confirmed);
        active_team.update(database).await?;

        Self::clear_confirmation_code(database, team.id).await?;

        Ok(())
    }

    pub async fn clear_confirmation_code(
        database: &DatabaseConnection,
        id: Uuid,
    ) -> Result<(), Error> {
        let mut team: ActiveModel = teams::Entity::find_by_id(id)
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?
            .into();

        team.confirmation_code = Set(None);
        team.update(database).await?;

        Ok(())
    }

    pub async fn generate_confirmation_code(
        database: &DatabaseConnection,
        id: Uuid,
    ) -> Result<(), Error> {
        let mut team: ActiveModel = teams::Entity::find_by_id(id)
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?
            .into();

        team.confirmation_code = Set(Some(Uuid::new_v4()));
        team.update(database).await?;

        Ok(())
    }

    pub async fn get_completed_tasks(
        database: &DatabaseConnection,
        team_id: Uuid,
    ) -> Result<Vec<String>, Error> {
        let tasks = flag_capture::Entity::find()
            .filter(flag_capture::Column::Team.eq(team_id))
            .all(database)
            .await?
            .into_iter()
            .map(|flag_capture| flag_capture.task)
            .collect();

        Ok(tasks)
    }
}

impl FromRequest for teams::Model {
    type Error = Error;

    type Future = future::Ready<Result<Self, Self::Error>>;

    fn from_request(req: &HttpRequest, _payload: &mut Payload) -> Self::Future {
        match req.extensions().get::<teams::Model>() {
            Some(data) => future::ready(Ok(data.clone())),
            None => future::ready(Err(Error::MissingExtension {
                name: "team".to_string(),
            })),
        }
    }
}
