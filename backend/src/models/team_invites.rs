use crate::entities::{team_invites, teams, users};
use crate::models::task::RegistrationConfig;
use crate::routes::teams::TeamError::{
    UserAlreadyBelongsToTeam, UserAlreadyInvited, UserDoesntHaveAnyInvitations,
    UserDoesntHaveInvitationsFromTeam,
};
use crate::utils::error::Error;
use sea_orm::ActiveValue::Set;
use sea_orm::QueryFilter;
use sea_orm::{
    ActiveModelTrait, DatabaseConnection, EntityTrait, PaginatorTrait, TransactionTrait,
};
use sea_orm::{ColumnTrait, ModelTrait};
use uuid::Uuid;

impl team_invites::Model {
    pub async fn invite_user(
        database: &DatabaseConnection,
        registration_config: &RegistrationConfig,
        invited_user: users::Model,
        team: teams::Model,
    ) -> Result<(), Error> {
        if let Some(team) = invited_user.get_team(database).await? {
            return Err(Error::Team(UserAlreadyBelongsToTeam {
                team_name: team.name,
            }));
        }

        teams::Model::assert_team_size_before_adding_user(database, registration_config, &team.id)
            .await?;

        Self::assert_user_doesnt_have_invites_from_this_team(
            database,
            invited_user.clone(),
            team.clone(),
        )
        .await?;

        team_invites::Entity::insert(team_invites::ActiveModel {
            user: Set(invited_user.id),
            team: Set(team.id),
            ..Default::default()
        })
        .exec(database)
        .await?;

        Ok(())
    }

    pub async fn get_invitations(
        database: &DatabaseConnection,
        user: users::Model,
    ) -> Result<Vec<team_invites::Model>, Error> {
        let invitations = team_invites::Entity::find()
            .filter(team_invites::Column::User.eq(user.id))
            .all(database)
            .await?;

        Ok(invitations)
    }

    pub async fn get_invitations_with_teams(
        database: &DatabaseConnection,
        user: users::Model,
    ) -> Result<Vec<(team_invites::Model, Option<teams::Model>)>, Error> {
        let invitations = team_invites::Entity::find()
            .filter(team_invites::Column::User.eq(user.id))
            .find_also_related(teams::Entity)
            .all(database)
            .await?;

        Ok(invitations)
    }

    pub async fn accept_invitation(
        database: &DatabaseConnection,
        registration_config: &RegistrationConfig,
        team: teams::Model,
        user: users::Model,
    ) -> Result<(), Error> {
        if Some(team.id) == user.team {
            return Err(Error::Team(UserAlreadyBelongsToTeam {
                team_name: team.name,
            }));
        }

        let invitations = Self::get_invitations(database, user.clone()).await?;

        if invitations.is_empty() {
            return Err(Error::Team(UserDoesntHaveAnyInvitations));
        }

        if !invitations.iter().any(|invite| invite.team == team.id) {
            return Err(Error::Team(UserDoesntHaveInvitationsFromTeam {
                team_name: team.name,
            }));
        }

        teams::Model::assert_team_size_before_adding_user(database, registration_config, &team.id)
            .await?;

        let transaction = database.begin().await?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.team = Set(Some(team.id));
        active_user.update(&transaction).await?;

        for invite in invitations {
            let active_invite: team_invites::ActiveModel = invite.into();
            active_invite.delete(&transaction).await?;
        }

        transaction.commit().await?;

        Ok(())
    }

    pub async fn assert_user_doesnt_have_invites_from_this_team(
        database: &DatabaseConnection,
        user: users::Model,
        team: teams::Model,
    ) -> Result<(), Error> {
        let invitation = team_invites::Entity::find()
            .filter(
                team_invites::Column::User
                    .eq(user.id)
                    .and(team_invites::Column::Team.eq(team.id)),
            )
            .count(database)
            .await?;

        if invitation > 0 {
            return Err(Error::Team(UserAlreadyInvited));
        }

        Ok(())
    }

    pub async fn get_invited_users(
        database: &DatabaseConnection,
        team: teams::Model,
    ) -> Result<Vec<String>, Error> {
        let invitations = team_invites::Entity::find()
            .filter(team_invites::Column::Team.eq(team.id))
            .find_with_related(users::Entity)
            .all(database)
            .await?;

        let users = invitations
            .into_iter()
            .map(|(_, user)| user.into_iter().next().unwrap().username)
            .collect();

        Ok(users)
    }

    pub async fn revoke_invitation(
        database: &DatabaseConnection,
        username: &str,
        team_id: Uuid,
    ) -> Result<(), Error> {
        let user = users::Model::find_by_username(database, username)
            .await?
            .ok_or(Error::UserNotFound)?;

        let invitation = team_invites::Entity::find()
            .filter(
                team_invites::Column::User
                    .eq(user.id)
                    .and(team_invites::Column::Team.eq(team_id)),
            )
            .one(database)
            .await?
            .ok_or(Error::Team(UserDoesntHaveAnyInvitations))?;

        invitation.delete(database).await?;

        Ok(())
    }
}
