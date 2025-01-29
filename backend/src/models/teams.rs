use crate::models::entities::{team_invites, teams, users};
use crate::routes::auth::AuthError::UserNotFound;
use crate::routes::teams::management::add_user::AddUserModel;
use crate::routes::teams::management::change_leader::ChangeLeaderModel;
use crate::routes::teams::management::change_name::ChangeNameModel;
use crate::routes::teams::management::create_team::CreateTeamModel;
use crate::routes::teams::management::remove_user::RemoveUserModel;
use crate::routes::teams::view_team::TeamWithMembers;
use crate::routes::teams::TeamError::{
    AlreadyExists, TeamNotFound, UserAlreadyBelongsToTeam, UserDoesntBelongToAnyTeam,
    UserDoesntBelongToYourTeam, UserDoesntHaveAnyInvitations,
};
use crate::utils::error::Error;
use crate::utils::jwt::Claims;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, QueryFilter};
use sea_orm::{ColumnTrait, DatabaseConnection, EntityTrait, TransactionTrait};

impl teams::Model {
    pub async fn create_team(
        database: &DatabaseConnection,
        create_team_json: CreateTeamModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        println!("a {:?}", claim_data);
        let user = users::Entity::find_by_id(claim_data.id)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        println!("b");
        if let Some(team_name) = user.team_name {
            return Err(Error::Team(UserAlreadyBelongsToTeam { team_name }));
        }

        println!("c");
        if teams::Entity::find()
            .filter(teams::Column::Name.eq(&create_team_json.team_name))
            .one(database)
            .await?
            .is_some()
        {
            return Err(Error::Team(AlreadyExists));
        }

        println!("d");
        let transaction = database.begin().await?;

        println!("e");
        let team_name = create_team_json.team_name;

        println!("f");
        teams::Entity::insert(teams::ActiveModel {
            name: Set(team_name.clone()),
            leader_name: Set(user.username.clone()),
            ..Default::default()
        })
        .exec(&transaction)
        .await?;

        println!("g");
        let mut active_user: users::ActiveModel = user.into();
        active_user.team_name = Set(Some(team_name.clone()));
        active_user.leads = Set(Some(team_name));
        active_user.update(&transaction).await?;

        println!("h");
        transaction.commit().await?;

        println!("i");
        Ok(())
    }
    pub async fn invite_user(
        database: &DatabaseConnection,
        add_user_json: AddUserModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find_by_id(claim_data.id)
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if user.team_name.is_none() {
            return Err(Error::Team(UserDoesntBelongToAnyTeam));
        }

        let team = teams::Entity::find()
            .filter(teams::Column::Name.eq(user.team_name.unwrap()))
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;

        if user.username != team.leader_name {
            return Err(Error::Unauthorized);
        }

        let invited_user = users::Entity::find()
            .filter(users::Column::Username.eq(&add_user_json.username))
            .one(database)
            .await?
            .ok_or(Error::Auth(UserNotFound))?;

        if let Some(team_name) = invited_user.team_name {
            return Err(Error::Team(UserAlreadyBelongsToTeam { team_name }));
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
            .ok_or(Error::Auth(UserNotFound))?;

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

        if user.team_name.is_some() {
            return Err(Error::Team(UserAlreadyBelongsToTeam {
                team_name: user.team_name.unwrap(),
            }));
        }

        let invitations = team_invites::Entity::find()
            .filter(team_invites::Column::User.eq(user.id))
            .all(database)
            .await?;

        if invitations.is_empty() {
            return Err(Error::Team(UserDoesntHaveAnyInvitations));
        }

        let mut active_user: users::ActiveModel = user.into();
        active_user.team_name = Set(Some(team_name.clone()));

        Ok(())
    }
    pub async fn get_team(
        database: &DatabaseConnection,
        team_name: String,
    ) -> Result<TeamWithMembers, Error> {
        let team_model = teams::Entity::find()
            .filter(teams::Column::Name.contains(team_name))
            .find_also_related(users::Entity)
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound));

        let (team, users) = team_model?;

        let member_names: Vec<String> = users.into_iter().map(|user| user.username).collect();

        let team_response = TeamWithMembers {
            team_name: team.name,
            leader_name: team.leader_name,
            created_at: team.created_at,
            members: member_names,
        };
        Ok(team_response)
    }
    pub async fn remove_user(
        database: &DatabaseConnection,
        remove_user_json: RemoveUserModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(claim_data.email))
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if user.leads.is_none() {
            return Err(Error::Unauthorized);
        }

        let team_name = user.leads.unwrap();

        let removed_user = users::Entity::find()
            .filter(users::Column::Username.eq(remove_user_json.username))
            .one(database)
            .await?
            .ok_or(Error::Auth(UserNotFound))?;

        if removed_user.team_name.is_none() {
            return Err(Error::Team(UserDoesntBelongToAnyTeam));
        }

        if removed_user.team_name.clone().unwrap() != team_name {
            return Err(Error::Team(UserDoesntBelongToYourTeam));
        }

        let transaction = database.begin().await?;

        let mut active_user: users::ActiveModel = removed_user.into();
        active_user.team_name = Set(None);
        active_user.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
    pub async fn change_name(
        database: &DatabaseConnection,
        new_team_name_json: ChangeNameModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(claim_data.email))
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if user.leads.is_none() {
            return Err(Error::Unauthorized);
        };

        let team = teams::Entity::find()
            .filter(teams::Column::Name.eq(user.leads.clone().unwrap()))
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;

        let transaction = database.begin().await?;

        let mut active_team: teams::ActiveModel = team.into();
        active_team.name = Set(new_team_name_json.new_name.clone());
        active_team.update(&transaction).await?;

        let mut active_user: users::ActiveModel = user.into();
        active_user.team_name = Set(Some(new_team_name_json.new_name.clone()));
        active_user.leads = Set(Some(new_team_name_json.new_name));
        active_user.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
    pub async fn change_leader(
        database: &DatabaseConnection,
        new_leader_username_json: ChangeLeaderModel,
        claim_data: Claims,
    ) -> Result<(), Error> {
        let user = users::Entity::find()
            .filter(users::Column::Email.eq(claim_data.email))
            .one(database)
            .await?
            .ok_or(Error::Unauthorized)?;

        if user.leads.is_none() {
            return Err(Error::Unauthorized);
        };

        let team = teams::Entity::find()
            .filter(teams::Column::Name.eq(user.leads.clone().unwrap()))
            .one(database)
            .await?
            .ok_or(Error::Team(TeamNotFound))?;

        let new_leader = users::Entity::find()
            .filter(users::Column::Username.eq(&new_leader_username_json.new_leader_username))
            .one(database)
            .await?
            .ok_or(Error::Auth(UserNotFound))?;

        let transaction = database.begin().await?;

        let mut active_team: teams::ActiveModel = team.into();
        active_team.leader_name = Set(new_leader_username_json.new_leader_username);
        active_team.update(&transaction).await?;

        let mut active_user: users::ActiveModel = new_leader.into();
        active_user.leads = Set(Some(user.leads.clone().unwrap()));
        active_user.update(&transaction).await?;

        transaction.commit().await?;

        Ok(())
    }
}
