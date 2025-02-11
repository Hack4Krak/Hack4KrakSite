use crate::entities::{teams, users};
use crate::routes::teams::TeamError::*;
use crate::utils::error::Error;
use actix_web::dev::Payload;
use actix_web::{FromRequest, HttpMessage, HttpRequest};
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, PaginatorTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, TransactionTrait};
use std::future;
use uuid::Uuid;

pub const MAX_MEMBERS_PER_TEAM: u8 = 5;

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
        name: &str,
    ) -> Result<(), Error> {
        let members_count = users::Entity::find()
            .filter(users::Column::Team.eq(name))
            .count(database)
            .await?;

        if members_count >= MAX_MEMBERS_PER_TEAM.into() {
            return Err(Error::Team(TeamIsFull {
                max_size: MAX_MEMBERS_PER_TEAM,
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
        let mut active_team: teams::ActiveModel = team.into();
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
