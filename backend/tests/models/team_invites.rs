use crate::test_utils::database::TestDatabase;
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{team_invites, teams, users};
use hack4krak_backend::models::task::RegistrationConfig;
use hack4krak_backend::routes::teams::TeamError;
use hack4krak_backend::utils::error::Error;
use sea_orm::EntityTrait;

#[tokio::test]
async fn invite_user_already_in_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let other_team = test_db
        .with_team(teams::UpdatableModel {
            name: Some("OtherTeam".to_string()),
            ..Default::default()
        })
        .await;
    let user = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;
    let config = RegistrationConfig::default();

    let result =
        team_invites::Model::invite_user(&test_db.database, &config, user, other_team).await;
    assert!(matches!(
        result.unwrap_err(),
        Error::Team(TeamError::UserAlreadyBelongsToTeam { .. })
    ));
}

#[tokio::test]
async fn get_invitations_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;

    test_db.with_invite(user.id, team.id).await;

    let invitations = team_invites::Model::get_invitations(&test_db.database, user)
        .await
        .unwrap();
    assert_eq!(invitations.len(), 1);
}

#[tokio::test]
async fn get_invitations_empty() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let invitations = team_invites::Model::get_invitations(&test_db.database, user)
        .await
        .unwrap();
    assert!(invitations.is_empty());
}

#[tokio::test]
async fn get_invitations_with_teams_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;

    test_db.with_invite(user.id, team.id).await;

    let invitations = team_invites::Model::get_invitations_with_teams(&test_db.database, user)
        .await
        .unwrap();
    assert_eq!(invitations.len(), 1);
    let (_, related_team) = &invitations[0];
    assert_eq!(related_team.as_ref().unwrap().id, team.id);
}

#[tokio::test]
async fn accept_invitation_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;
    let user = test_db.with_default_user().await;
    let config = RegistrationConfig::default();

    test_db.with_invite(user.id, team.id).await;

    let result = team_invites::Model::accept_invitation(
        &test_db.database,
        &config,
        team.clone(),
        user.clone(),
    )
    .await;
    assert!(result.is_ok());

    let updated_user: users::Model = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_user.team, Some(team.id));

    let invitations = team_invites::Model::get_invitations(&test_db.database, updated_user)
        .await
        .unwrap();
    assert!(invitations.is_empty());
}

#[tokio::test]
async fn accept_invitation_already_in_same_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;
    let config = RegistrationConfig::default();

    let result =
        team_invites::Model::accept_invitation(&test_db.database, &config, team, user).await;
    assert!(matches!(
        result.unwrap_err(),
        Error::Team(TeamError::UserAlreadyBelongsToTeam { .. })
    ));
}

#[tokio::test]
async fn accept_invitation_no_invitations() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;
    let config = RegistrationConfig::default();

    let result =
        team_invites::Model::accept_invitation(&test_db.database, &config, team, user).await;
    assert!(matches!(
        result.unwrap_err(),
        Error::Team(TeamError::UserDoesntHaveAnyInvitations)
    ));
}

#[tokio::test]
async fn assert_user_doesnt_have_invites_from_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;

    let result = team_invites::Model::assert_user_doesnt_have_invites_from_this_team(
        &test_db.database,
        user,
        team,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn assert_user_has_invites_from_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;

    test_db.with_invite(user.id, team.id).await;

    let result = team_invites::Model::assert_user_doesnt_have_invites_from_this_team(
        &test_db.database,
        user,
        team,
    )
    .await;
    assert!(matches!(
        result.unwrap_err(),
        Error::Team(TeamError::UserAlreadyInvited)
    ));
}

#[tokio::test]
async fn get_invited_users_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;

    test_db.with_invite(user.id, team.id).await;

    let invited = team_invites::Model::get_invited_users(&test_db.database, team)
        .await
        .unwrap();
    assert_eq!(invited.len(), 1);
    assert_eq!(invited[0], user.username);
}

#[tokio::test]
async fn revoke_invitation_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;

    test_db.with_invite(user.id, team.id).await;

    let result =
        team_invites::Model::revoke_invitation(&test_db.database, &user.username, team.id).await;
    assert!(result.is_ok());

    let invitations = team_invites::Model::get_invitations(&test_db.database, user)
        .await
        .unwrap();
    assert!(invitations.is_empty());
}

#[tokio::test]
async fn revoke_invitation_user_not_found() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let result =
        team_invites::Model::revoke_invitation(&test_db.database, "nonexistent", team.id).await;
    assert!(matches!(result.unwrap_err(), Error::UserNotFound));
}

#[tokio::test]
async fn revoke_invitation_no_invitation() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;

    let result =
        team_invites::Model::revoke_invitation(&test_db.database, &user.username, team.id).await;
    assert!(matches!(
        result.unwrap_err(),
        Error::Team(TeamError::UserDoesntHaveAnyInvitations)
    ));
}
