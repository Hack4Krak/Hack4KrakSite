use crate::test_utils::database::TestDatabase;
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{flag_capture, teams, users};
use hack4krak_backend::models::task::RegistrationConfig;
use sea_orm::ActiveValue::Set;
use sea_orm::EntityTrait;
use uuid::Uuid;

#[tokio::test]
async fn find_by_name_existing() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let found = teams::Model::find_by_name(&test_db.database, &team.name)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, team.id);
}

#[tokio::test]
async fn find_by_name_non_existing() {
    let test_db = TestDatabase::new().await;

    let found = teams::Model::find_by_name(&test_db.database, "NonExistent")
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn assert_team_size_within_limit() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let config = RegistrationConfig {
        max_team_size: 5,
        ..Default::default()
    };

    let result =
        teams::Model::assert_team_size_before_adding_user(&test_db.database, &config, &team.id)
            .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn assert_team_size_exceeds_limit() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let config = RegistrationConfig {
        max_team_size: 1,
        ..Default::default()
    };

    // Add a user to the team first
    test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let result =
        teams::Model::assert_team_size_before_adding_user(&test_db.database, &config, &team.id)
            .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn try_create_success() {
    let test_db = TestDatabase::new().await;

    let result = teams::Model::try_create(&test_db.database, "NewTeam".to_string(), None).await;
    assert!(result.is_ok());

    let team = teams::Model::find_by_name(&test_db.database, "NewTeam")
        .await
        .unwrap();
    assert!(team.is_some());
}

#[tokio::test]
async fn try_create_duplicate() {
    let test_db = TestDatabase::new().await;
    test_db.with_default_team().await;

    let result = teams::Model::try_create(&test_db.database, "Dziengiel".to_string(), None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn create_team_and_assign_leader() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let result = teams::Model::create(
        &test_db.database,
        "TestTeam".to_string(),
        None,
        user.clone(),
    )
    .await;
    assert!(result.is_ok());

    let updated_user = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(updated_user.is_leader);
    assert!(updated_user.team.is_some());
}

#[tokio::test]
async fn kick_user_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let leader = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let member = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let result = teams::Model::kick_user(&test_db.database, member.clone(), leader).await;
    assert!(result.is_ok());

    let kicked_user = users::Entity::find_by_id(member.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(kicked_user.team.is_none());
}

#[tokio::test]
async fn kick_user_cant_kick_yourself() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let user = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;

    let result = teams::Model::kick_user(&test_db.database, user.clone(), user.clone()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn kick_user_cant_kick_leader() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let leader = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let member = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let result = teams::Model::kick_user(&test_db.database, leader, member).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn kick_user_different_team() {
    let test_db = TestDatabase::new().await;
    let team1 = test_db.with_default_team().await;
    let team2 = test_db
        .with_team(teams::UpdatableModel {
            name: Some("OtherTeam".to_string()),
            ..Default::default()
        })
        .await;

    let leader = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team1.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let other_member = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team2.id)),
            ..Default::default()
        })
        .await;

    let result = teams::Model::kick_user(&test_db.database, other_member, leader).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn remove_user_from_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let user = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let result = teams::Model::remove_user(&test_db.database, user.clone()).await;
    assert!(result.is_ok());

    let removed = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(removed.team.is_none());
}

#[tokio::test]
async fn rename_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let result =
        teams::Model::rename(&test_db.database, "RenamedTeam".to_string(), team.clone()).await;
    assert!(result.is_ok());

    let renamed = teams::Entity::find_by_id(team.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(renamed.name, "RenamedTeam");
}

#[tokio::test]
async fn change_leader_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let old_leader = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let new_leader = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let result =
        teams::Model::change_leader(&test_db.database, new_leader.clone(), old_leader.clone())
            .await;
    assert!(result.is_ok());

    let updated_new = users::Entity::find_by_id(new_leader.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(updated_new.is_leader);

    let updated_old = users::Entity::find_by_id(old_leader.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(!updated_old.is_leader);
}

#[tokio::test]
async fn change_leader_different_team() {
    let test_db = TestDatabase::new().await;
    let team1 = test_db.with_default_team().await;
    let team2 = test_db
        .with_team(teams::UpdatableModel {
            name: Some("OtherTeam".to_string()),
            ..Default::default()
        })
        .await;

    let leader = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team1.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let other_user = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team2.id)),
            ..Default::default()
        })
        .await;

    let result = teams::Model::change_leader(&test_db.database, other_user, leader).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn delete_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let leader = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;

    let result = teams::Model::delete(&test_db.database, leader.clone(), team.clone()).await;
    assert!(result.is_ok());

    let deleted = teams::Entity::find_by_id(team.id)
        .one(&test_db.database)
        .await
        .unwrap();
    assert!(deleted.is_none());

    let user_after = users::Entity::find_by_id(leader.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(!user_after.is_leader);
}

#[tokio::test]
async fn leader_found() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let leader = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;

    let found = teams::Model::leader(&test_db.database, team.id).await;
    assert!(found.is_ok());
    assert_eq!(found.unwrap().id, leader.id);
}

#[tokio::test]
async fn leader_not_found() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let found = teams::Model::leader(&test_db.database, team.id).await;
    assert!(found.is_err());
}

#[tokio::test]
async fn list_teams_empty() {
    let test_db = TestDatabase::new().await;

    let list = teams::Model::list(&test_db.database).await.unwrap();
    assert!(list.is_empty());
}

#[tokio::test]
async fn list_teams_with_members() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let list = teams::Model::list(&test_db.database).await.unwrap();
    assert_eq!(list.len(), 1);
    assert_eq!(list[0].members.len(), 1);
    assert_eq!(list[0].team_name, team.name);
}

#[tokio::test]
async fn delete_as_admin_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;

    let result = teams::Model::delete_as_admin(&test_db.database, team.id).await;
    assert!(result.is_ok());

    let deleted = teams::Entity::find_by_id(team.id)
        .one(&test_db.database)
        .await
        .unwrap();
    assert!(deleted.is_none());
}

#[tokio::test]
async fn delete_as_admin_not_found() {
    let test_db = TestDatabase::new().await;

    let result = teams::Model::delete_as_admin(&test_db.database, Uuid::new_v4()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn confirm_team_success() {
    let test_db = TestDatabase::new().await;
    let code = Uuid::new_v4();
    let team = test_db
        .with_team(teams::UpdatableModel {
            confirmation_code: Some(Some(code)),
            ..Default::default()
        })
        .await;

    let result = teams::Model::confirm(&test_db.database, code).await;
    assert!(result.is_ok());

    let confirmed = teams::Entity::find_by_id(team.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(confirmed.status, TeamStatus::Confirmed);
    assert!(confirmed.confirmation_code.is_none());
}

#[tokio::test]
async fn confirm_team_invalid_code() {
    let test_db = TestDatabase::new().await;

    let result = teams::Model::confirm(&test_db.database, Uuid::new_v4()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn clear_confirmation_code_success() {
    let test_db = TestDatabase::new().await;
    let code = Uuid::new_v4();
    let team = test_db
        .with_team(teams::UpdatableModel {
            confirmation_code: Some(Some(code)),
            ..Default::default()
        })
        .await;

    let result = teams::Model::clear_confirmation_code(&test_db.database, team.id).await;
    assert!(result.is_ok());

    let updated = teams::Entity::find_by_id(team.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(updated.confirmation_code.is_none());
}

#[tokio::test]
async fn clear_confirmation_code_team_not_found() {
    let test_db = TestDatabase::new().await;

    let result = teams::Model::clear_confirmation_code(&test_db.database, Uuid::new_v4()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn generate_confirmation_code_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    assert!(team.confirmation_code.is_none());

    let result = teams::Model::generate_confirmation_code(&test_db.database, team.id).await;
    assert!(result.is_ok());

    let updated = teams::Entity::find_by_id(team.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(updated.confirmation_code.is_some());
}

#[tokio::test]
async fn generate_confirmation_code_team_not_found() {
    let test_db = TestDatabase::new().await;

    let result = teams::Model::generate_confirmation_code(&test_db.database, Uuid::new_v4()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_completed_tasks_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    flag_capture::Entity::insert(flag_capture::ActiveModel {
        team: Set(team.id),
        task: Set("task-1".to_string()),
        submitted_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    })
    .exec(&test_db.database)
    .await
    .unwrap();

    let tasks = teams::Model::get_completed_tasks(&test_db.database, team.id)
        .await
        .unwrap();
    assert_eq!(tasks.len(), 1);
    assert_eq!(tasks[0], "task-1");
}

#[tokio::test]
async fn get_completed_tasks_empty() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    let tasks = teams::Model::get_completed_tasks(&test_db.database, team.id)
        .await
        .unwrap();
    assert!(tasks.is_empty());
}

#[tokio::test]
async fn get_tasks_by_team_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    flag_capture::Entity::insert(flag_capture::ActiveModel {
        team: Set(team.id),
        task: Set("task-1".to_string()),
        submitted_at: Set(chrono::Utc::now().naive_utc()),
        ..Default::default()
    })
    .exec(&test_db.database)
    .await
    .unwrap();

    let tasks = teams::Model::get_tasks_by_team(&test_db.database, vec![team.id])
        .await
        .unwrap();

    assert!(tasks.contains_key(&team.id));
    assert!(tasks[&team.id].contains_key("task-1"));
}

#[tokio::test]
async fn assert_is_confirmed_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;

    assert!(team.assert_is_confirmed().is_ok());
}

#[tokio::test]
async fn assert_is_confirmed_absent() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;

    assert!(team.assert_is_confirmed().is_err());
}
