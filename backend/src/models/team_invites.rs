use crate::entities::{team_invites, teams, users};
use crate::routes::teams::TeamError::{
    UserAlreadyBelongsToTeam, UserDoesntHaveAnyInvitations, UserDoesntHaveInvitationsFromTeam,
};
use crate::utils::error::Error;
use sea_orm::ActiveValue::Set;
use sea_orm::ColumnTrait;
use sea_orm::QueryFilter;
use sea_orm::{ActiveModelTrait, DatabaseConnection, EntityTrait, TransactionTrait};

impl team_invites::Model {
    pub async fn invite_user(
        database: &DatabaseConnection,
        invited_user: users::Model,
        team: teams::Model,
    ) -> Result<(), Error> {
        if let Some(team) = invited_user.get_team(database).await? {
            return Err(Error::Team(UserAlreadyBelongsToTeam {
                team_name: team.name,
            }));
        }

        teams::Model::assert_correct_team_size(database, &team.id).await?;

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

        teams::Model::assert_correct_team_size(database, &team.id).await?;

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
}
