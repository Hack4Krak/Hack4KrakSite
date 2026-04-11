use crate::test_utils::database::TestDatabase;
use hack4krak_backend::entities::sea_orm_active_enums::TeamStatus;
use hack4krak_backend::entities::{external_team_invitation, teams, users};
use hack4krak_backend::models::task::RegistrationConfig;
use sea_orm::{ColumnTrait, EntityTrait, QueryFilter};
use uuid::Uuid;

#[tokio::test]
async fn create_generates_alphanumeric_code() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin_code = Uuid::new_v4();

    let code =
        external_team_invitation::Model::create(&test_db.database, team.id, admin_code).await;
    assert!(code.is_ok());

    let code = code.unwrap();
    assert_eq!(code.len(), 6);
    assert!(code.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[tokio::test]
async fn create_stores_invitation_in_database() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin_code = Uuid::new_v4();

    let code = external_team_invitation::Model::create(&test_db.database, team.id, admin_code)
        .await
        .unwrap();

    let stored = external_team_invitation::Entity::find()
        .filter(external_team_invitation::Column::AccessCode.eq(&code))
        .one(&test_db.database)
        .await
        .unwrap();

    assert!(stored.is_some());
    let stored = stored.unwrap();
    assert_eq!(stored.team, team.id);
    assert_eq!(stored.administration_code, admin_code);
    assert_eq!(stored.access_code, code);
}

#[tokio::test]
async fn create_generates_unique_codes() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin_code = Uuid::new_v4();

    let mut codes = Vec::new();
    for _ in 0..10 {
        let code = external_team_invitation::Model::create(&test_db.database, team.id, admin_code)
            .await
            .unwrap();
        codes.push(code);
    }

    codes.sort();
    codes.dedup();
    assert_eq!(codes.len(), 10, "All generated codes should be unique");
}

#[tokio::test]
async fn accept_invitation_assigns_user_to_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;
    let user = test_db.with_default_user().await;
    let admin_code = Uuid::new_v4();
    let config = RegistrationConfig::default();

    let code = external_team_invitation::Model::create(&test_db.database, team.id, admin_code)
        .await
        .unwrap();

    external_team_invitation::Model::accept_invitation(
        &test_db.database,
        &config,
        code,
        user.clone(),
    )
    .await
    .unwrap();

    let updated_user = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated_user.team, Some(team.id));
}

#[tokio::test]
async fn accept_invitation_removes_invitation_from_database() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db.with_default_user().await;
    let admin_code = Uuid::new_v4();
    let config = RegistrationConfig::default();

    let code = external_team_invitation::Model::create(&test_db.database, team.id, admin_code)
        .await
        .unwrap();

    external_team_invitation::Model::accept_invitation(
        &test_db.database,
        &config,
        code.clone(),
        user,
    )
    .await
    .unwrap();

    let invitation = external_team_invitation::Entity::find()
        .filter(external_team_invitation::Column::AccessCode.eq(&code))
        .one(&test_db.database)
        .await
        .unwrap();
    assert!(invitation.is_none());
}

#[tokio::test]
async fn accept_invitation_invalid_code_returns_error() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;
    let config = RegistrationConfig::default();

    let result = external_team_invitation::Model::accept_invitation(
        &test_db.database,
        &config,
        "NONEXISTENT".to_string(),
        user,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn accept_invitation_respects_team_size_limit() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin_code = Uuid::new_v4();

    let config = RegistrationConfig {
        max_team_size: 1,
        ..Default::default()
    };

    // Add a user already in the team
    test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            email: Some("existing@example.com".to_string()),
            username: Some("existing_user".to_string()),
            ..Default::default()
        })
        .await;

    let new_user = test_db
        .with_user(users::UpdatableModel {
            email: Some("new@example.com".to_string()),
            username: Some("new_user".to_string()),
            ..Default::default()
        })
        .await;

    let code = external_team_invitation::Model::create(&test_db.database, team.id, admin_code)
        .await
        .unwrap();

    let result = external_team_invitation::Model::accept_invitation(
        &test_db.database,
        &config,
        code,
        new_user,
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn list_returns_invitations_for_admin_code() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin_code = Uuid::new_v4();

    external_team_invitation::Model::create(&test_db.database, team.id, admin_code)
        .await
        .unwrap();
    external_team_invitation::Model::create(&test_db.database, team.id, admin_code)
        .await
        .unwrap();

    let results = external_team_invitation::Model::list(&test_db.database, admin_code)
        .await
        .unwrap();
    assert_eq!(results.len(), 2);

    for (_, team_name) in &results {
        assert_eq!(team_name, &team.name);
    }
}

#[tokio::test]
async fn list_returns_empty_for_unknown_admin_code() {
    let test_db = TestDatabase::new().await;

    let results = external_team_invitation::Model::list(&test_db.database, Uuid::new_v4())
        .await
        .unwrap();
    assert!(results.is_empty());
}

#[tokio::test]
async fn list_only_returns_invitations_for_given_admin_code() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin_code_a = Uuid::new_v4();
    let admin_code_b = Uuid::new_v4();

    external_team_invitation::Model::create(&test_db.database, team.id, admin_code_a)
        .await
        .unwrap();
    external_team_invitation::Model::create(&test_db.database, team.id, admin_code_b)
        .await
        .unwrap();

    let results = external_team_invitation::Model::list(&test_db.database, admin_code_a)
        .await
        .unwrap();
    assert_eq!(results.len(), 1);
}

#[tokio::test]
async fn grouped_codes_groups_by_team_name() {
    let test_db = TestDatabase::new().await;
    let team_a = test_db.with_default_team().await;
    let team_b = test_db
        .with_team(teams::UpdatableModel {
            name: Some("TeamB".to_string()),
            ..Default::default()
        })
        .await;
    let admin_code = Uuid::new_v4();

    external_team_invitation::Model::create(&test_db.database, team_a.id, admin_code)
        .await
        .unwrap();
    external_team_invitation::Model::create(&test_db.database, team_a.id, admin_code)
        .await
        .unwrap();
    external_team_invitation::Model::create(&test_db.database, team_b.id, admin_code)
        .await
        .unwrap();

    let grouped = external_team_invitation::Model::grouped_codes(&test_db.database, admin_code)
        .await
        .unwrap();

    assert_eq!(grouped.len(), 2);

    let serialized = serde_json::to_value(&grouped).unwrap();
    let array = serialized.as_array().unwrap();

    let team_a_group = array
        .iter()
        .find(|g| g["team_name"].as_str() == Some(&team_a.name))
        .unwrap();
    assert_eq!(team_a_group["codes"].as_array().unwrap().len(), 2);

    let team_b_group = array
        .iter()
        .find(|g| g["team_name"].as_str() == Some("TeamB"))
        .unwrap();
    assert_eq!(team_b_group["codes"].as_array().unwrap().len(), 1);
}

#[tokio::test]
async fn grouped_codes_returns_empty_for_unknown_admin_code() {
    let test_db = TestDatabase::new().await;

    let grouped = external_team_invitation::Model::grouped_codes(&test_db.database, Uuid::new_v4())
        .await
        .unwrap();
    assert!(grouped.is_empty());
}
