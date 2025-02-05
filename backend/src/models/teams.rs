use crate::models::entities::{team_invites, teams, users};
use crate::routes::teams::management::change_leader::ChangeLeaderModel;
use crate::routes::teams::management::create_team::CreateTeamModel;
use crate::routes::teams::management::invite_user::AddUserModel;
use crate::routes::teams::management::remove_user::RemoveUserModel;
use crate::routes::teams::management::rename::ChangeNameModel;
use crate::routes::teams::team::TeamWithMembers;
use crate::routes::teams::TeamError::{
    AlreadyExists, TeamIsFull, TeamLeaderCantLeaveTeam, TeamNotFound, UserAlreadyBelongsToTeam,
    UserCantRemoveTeamLeader, UserCantRemoveYourself, UserDoesntBelongToAnyTeam,
    UserDoesntBelongToYourTeam, UserDoesntHaveAnyInvitations, UserDoesntHaveInvitationsFromTeam,
    UserIsNotTeamLeader,
};
use crate::utils::error::Error;
use crate::utils::jwt::Claims;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, TransactionTrait};
use uuid::Uuid;

impl teams::Model {
    pub async fn create_team(
        database: &DatabaseConnection,
        create_team_json: CreateTeamModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find_by_id(claim_data.id)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if let Some(team_id) = user.team {
            let belonging_team = teams::Entity::find_by_id(team_id)
                .one(database)
                .await?
                .ok_or(Error::Team(TeamNotFound))?;
            return Err(Error::Team(UserAlreadyBelongsToTeam {
                team_name: belonging_team.name,
            }));
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

    pub async fn invite_user(
        database: &DatabaseConnection,
        add_user_username: AddUserModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find_by_id(claim_data.id)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        let Some(team_id) = user.team else {
            return Err(Error::Team(UserDoesntBelongToAnyTeam {
                username: user.username,
            }))?;
        };

        let team = teams::Entity::find_by_id(team_id)
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;

        if !user.is_leader {
            return Err(Error::Team(UserIsNotTeamLeader));
        }

        let invited_user = users::Entity::find()
            .filter(users::Column::Username.eq(&add_user_username.username))
            .one(database)
            .await?
            .ok_or(Error::UserNotFound)?;

        if let Some(team_id) = invited_user.team {
            let belonging_team = teams::Entity::find_by_id(team_id)
                .one(database)
                .await?
                .ok_or(Error::Team(TeamNotFound))?;
            return Err(Error::Team(UserAlreadyBelongsToTeam {
                team_name: belonging_team.name,
            }));
        }

        let members = users::Entity::find()
            .filter(users::Column::Team.eq(team.id))
            .all(database)
            .await?;

        if members.len() >= 5 {
            return Err(Error::Team(TeamIsFull));
        }

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
        claim_data: Claims,
    ) -> Result<Vec<team_invites::Model>, Error> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(claim_data.email))
            .one(database)
            .await?
            .ok_or(Error::UserNotFound)?;

        let invitations = team_invites::Entity::find()
            .filter(team_invites::Column::User.eq(user.id))
            .all(database)
            .await?;

        Ok(invitations)
    }

    pub async fn accept_invitation(
        database: &DatabaseConnection,
        team_name: String,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(claim_data.email))
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        let team = teams::Entity::find()
            .filter(teams::Column::Name.eq(team_name.to_string()))
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;

        if Some(team.id) == user.team {
            return Err(Error::Team(UserAlreadyBelongsToTeam { team_name }));
        }

        let invitations = team_invites::Entity::find()
            .filter(team_invites::Column::User.eq(user.id))
            .all(database)
            .await?;

        if invitations.is_empty() {
            return Err(Error::Team(UserDoesntHaveAnyInvitations));
        }

        if !invitations.iter().any(|invite| invite.team == team.id) {
            return Err(Error::Team(UserDoesntHaveInvitationsFromTeam { team_name }));
        }

        let members = users::Entity::find()
            .filter(users::Column::Team.eq(team.id))
            .all(database)
            .await?;

        if members.len() >= 5 {
            return Err(Error::Team(TeamIsFull));
        }

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

    pub async fn get_team(
        database: &DatabaseConnection,
        team_name: String,
    ) -> Result<TeamWithMembers, Error> {
        let team = teams::Entity::find()
            .filter(teams::Column::Name.eq(team_name))
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;

        let users = users::Entity::find()
            .filter(users::Column::Team.eq(team.id))
            .all(database)
            .await?;
        let member_names: Vec<String> = users.into_iter().map(|user| user.username).collect();

        let team_response = TeamWithMembers {
            team_name: team.name,
            created_at: team.created_at,
            members: member_names,
        };
        Ok(team_response)
    }

    pub async fn remove_user(
        database: &DatabaseConnection,
        remove_user_username: RemoveUserModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find_by_id(claim_data.id)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if !user.is_leader {
            return Err(Error::Team(UserIsNotTeamLeader));
        }

        let team_id = user.team;

        let removed_user = users::Entity::find()
            .filter(users::Column::Username.eq(remove_user_username.username))
            .one(database)
            .await?
            .ok_or(Error::UserNotFound)?;

        if removed_user.id == user.id {
            return Err(Error::Team(UserCantRemoveYourself));
        }

        if removed_user.team.is_none() {
            return Err(Error::Team(UserDoesntBelongToAnyTeam {
                username: removed_user.username,
            }));
        }

        if removed_user.team != team_id {
            return Err(Error::Team(UserDoesntBelongToYourTeam));
        }

        if removed_user.is_leader {
            return Err(Error::Team(UserCantRemoveTeamLeader));
        }

        let mut active_user: users::ActiveModel = removed_user.into();
        active_user.team = Set(None);
        active_user.update(database).await?;

        Ok(())
    }

    pub async fn leave_team(
        database: &DatabaseConnection,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find_by_id(claim_data.id)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if user.team.is_none() {
            return Err(Error::Team(UserDoesntBelongToAnyTeam {
                username: user.username,
            }));
        }

        if user.is_leader {
            return Err(Error::Team(TeamLeaderCantLeaveTeam));
        }

        let mut active_user: users::ActiveModel = user.into();
        active_user.team = Set(None);
        active_user.update(database).await?;

        Ok(())
    }

    pub async fn rename(
        database: &DatabaseConnection,
        new_team_name: ChangeNameModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find_by_id(claim_data.id)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if !user.is_leader {
            return Err(Error::Team(UserIsNotTeamLeader));
        };

        let team = teams::Entity::find()
            .filter(teams::Column::Name.eq(user.team.ok_or(Error::Team(
                UserDoesntBelongToAnyTeam {
                    username: user.username,
                },
            ))?))
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;

        let mut active_team: teams::ActiveModel = team.into();
        active_team.name = Set(new_team_name.new_name.clone());
        active_team.update(database).await?;

        Ok(())
    }

    pub async fn change_leader(
        database: &DatabaseConnection,
        new_leader_username: ChangeLeaderModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(claim_data.email))
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if !user.is_leader {
            return Err(Error::Team(UserIsNotTeamLeader));
        };

        let new_leader = users::Entity::find()
            .filter(users::Column::Username.eq(&new_leader_username.new_leader_username))
            .one(database)
            .await?
            .ok_or(Error::UserNotFound)?;

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
