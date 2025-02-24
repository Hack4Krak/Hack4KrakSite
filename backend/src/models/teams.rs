use crate::entities::teams::ActiveModel;
use crate::entities::{teams, users};
use crate::routes::admin::teams::update::UpdateTeamModel;
use crate::routes::teams::TeamError::*;
use crate::utils::error::Error;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use sea_orm::ActiveValue::Set;
use sea_orm::prelude::DateTime;
use sea_orm::{ActiveModelTrait, ModelTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, TransactionTrait};
use serde::{Deserialize, Serialize};
use std::future;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct TeamWithMembers {
    pub id: String,
    pub team_name: String,
    pub created_at: DateTime,
    pub members: Vec<String>,
}

impl teams::Model {
    pub async fn find_by_name(
        database: &DatabaseConnection,
        name: &str,
    ) -> Result<Option<Self>, Error> {
        let team = teams::Entity::find()
            .filter(teams::Column::Name.eq(name))
            .one(database)
            .await?;

        Ok(team)
    }

    pub async fn assert_correct_team_size(
        database: &DatabaseConnection,
        max_team_size: u16,
        id: &Uuid,
    ) -> Result<(), Error> {
        let members_count = users::Entity::find()
            .filter(users::Column::Team.eq(*id))
            .count(database)
            .await?;

        if members_count >= max_team_size as u64 {
            return Err(Error::Team(TeamIsFull {
                max_size: max_team_size,
            }));
        }

        Ok(())
    }

    pub async fn create(
        database: &DatabaseConnection,
        team_name: String,
        user: users::Model,
    ) -> Result<(), Error> {
        let transaction = database.begin().await?;

        let uuid = Uuid::new_v4();
        teams::Entity::insert(teams::ActiveModel {
            id: Set(uuid),
            name: Set(team_name),
            ..Default::default()
        })
        .exec(&transaction)
        .await?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.team = Set(Some(uuid));
        active_user.is_leader = Set(true);
        active_user.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
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
                let members_ids: Vec<String> =
                    users.into_iter().map(|user| user.id.to_string()).collect();
                TeamWithMembers {
                    id: team.id.to_string(),
                    team_name: team.name,
                    created_at: team.created_at,
                    members: members_ids,
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
                return Err(Error::Team(AlreadyExists));
            }
            let mut active_team: ActiveModel = team.into();
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
