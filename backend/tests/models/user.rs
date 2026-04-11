use crate::test_utils::database::TestDatabase;
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::users;
use hack4krak_backend::models::user::{UserInformation, validate_name_chars};
use sea_orm::EntityTrait;

#[tokio::test]
async fn find_by_username_found() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let found = users::Model::find_by_username(&test_db.database, &user.username)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user.id);
}

#[tokio::test]
async fn find_by_username_not_found() {
    let test_db = TestDatabase::new().await;

    let found = users::Model::find_by_username(&test_db.database, "nonexistent")
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn find_by_email_found() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let found = users::Model::find_by_email(&test_db.database, &user.email)
        .await
        .unwrap();
    assert!(found.is_some());
    assert_eq!(found.unwrap().id, user.id);
}

#[tokio::test]
async fn find_by_email_not_found() {
    let test_db = TestDatabase::new().await;

    let found = users::Model::find_by_email(&test_db.database, "nobody@example.com")
        .await
        .unwrap();
    assert!(found.is_none());
}

#[tokio::test]
async fn assert_is_unique_no_conflicts() {
    let test_db = TestDatabase::new().await;

    let result =
        users::Model::assert_is_unique(&test_db.database, "fresh@email.com", "fresh_user", None)
            .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn assert_is_unique_duplicate_email() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let result =
        users::Model::assert_is_unique(&test_db.database, &user.email, "other_name", None).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn assert_is_unique_duplicate_username() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let result =
        users::Model::assert_is_unique(&test_db.database, "other@email.com", &user.username, None)
            .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn assert_is_unique_same_user_id() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let result = users::Model::assert_is_unique(
        &test_db.database,
        &user.email,
        &user.username,
        Some(user.id),
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn assert_is_unique_different_user_id() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let result = users::Model::assert_is_unique(
        &test_db.database,
        &user.email,
        &user.username,
        Some(uuid::Uuid::new_v4()),
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn get_team_has_team() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let user = test_db
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let found_team = user.get_team(&test_db.database).await.unwrap();
    assert!(found_team.is_some());
    assert_eq!(found_team.unwrap().id, team.id);
}

#[tokio::test]
async fn get_team_no_team() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let found_team = user.get_team(&test_db.database).await.unwrap();
    assert!(found_team.is_none());
}

#[tokio::test]
async fn create_from_oauth_existing_user() {
    let test_db = TestDatabase::new().await;
    let existing = test_db.with_default_user().await;

    let user = users::Model::create_from_oauth(
        &test_db.database,
        "different_name".to_string(),
        existing.email.clone(),
    )
    .await
    .unwrap();

    assert_eq!(user.id, existing.id);
}

#[tokio::test]
async fn create_from_user_info_success() {
    let test_db = TestDatabase::new().await;

    let user_info = UserInformation {
        name: "new_user".to_string(),
        email: "new@example.com".to_string(),
        password_hash: "hashed_password".to_string(),
    };

    let result = users::Model::create_from_user_info(&test_db.database, user_info).await;
    assert!(result.is_ok());

    let found = users::Model::find_by_username(&test_db.database, "new_user")
        .await
        .unwrap();
    assert!(found.is_some());
}

#[tokio::test]
async fn delete_user_success() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    let target = test_db
        .with_user(users::UpdatableModel {
            email: Some("target@example.com".to_string()),
            username: Some("target_user".to_string()),
            ..Default::default()
        })
        .await;

    let result = users::Model::delete(&test_db.database, admin, target.id).await;
    assert!(result.is_ok());

    let deleted = users::Entity::find_by_id(target.id)
        .one(&test_db.database)
        .await
        .unwrap();
    assert!(deleted.is_none());
}

#[tokio::test]
async fn delete_user_insufficient_role() {
    let test_db = TestDatabase::new().await;
    let default_user = test_db.with_default_user().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            email: Some("admin@example.com".to_string()),
            username: Some("admin_user".to_string()),
            ..Default::default()
        })
        .await;

    let result = users::Model::delete(&test_db.database, default_user, admin.id).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn delete_user_not_found() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let result = users::Model::delete(&test_db.database, admin, uuid::Uuid::new_v4()).await;
    assert!(result.is_err());
}

#[tokio::test]
async fn update_user_success() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let result = users::Model::update(
        &test_db.database,
        user.clone(),
        users::UpdatableModel {
            username: Some("updated_name".to_string()),
            ..Default::default()
        },
    )
    .await;
    assert!(result.is_ok());

    let updated = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.username, "updated_name");
}

#[tokio::test]
async fn update_as_admin_success() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    let target = test_db
        .with_user(users::UpdatableModel {
            email: Some("target@example.com".to_string()),
            username: Some("target_user".to_string()),
            ..Default::default()
        })
        .await;

    let result = users::Model::update_as_admin(
        &test_db.database,
        admin,
        target.clone(),
        users::UpdatableModel {
            username: Some("admin_updated".to_string()),
            ..Default::default()
        },
    )
    .await;
    assert!(result.is_ok());

    let updated = users::Entity::find_by_id(target.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.username, "admin_updated");
}

#[tokio::test]
async fn update_as_admin_insufficient_role() {
    let test_db = TestDatabase::new().await;
    let default_user = test_db.with_default_user().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            email: Some("admin@example.com".to_string()),
            username: Some("admin_user".to_string()),
            ..Default::default()
        })
        .await;

    let result = users::Model::update_as_admin(
        &test_db.database,
        default_user,
        admin,
        users::UpdatableModel {
            username: Some("hacked".to_string()),
            ..Default::default()
        },
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn update_as_admin_role_change_requires_owner() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    let target = test_db
        .with_user(users::UpdatableModel {
            email: Some("target@example.com".to_string()),
            username: Some("target_user".to_string()),
            ..Default::default()
        })
        .await;

    let result = users::Model::update_as_admin(
        &test_db.database,
        admin,
        target,
        users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        },
    )
    .await;
    assert!(result.is_err());
}

#[tokio::test]
async fn update_as_admin_owner_can_change_roles() {
    let test_db = TestDatabase::new().await;
    let owner = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Owner),
            ..Default::default()
        })
        .await;
    let target = test_db
        .with_user(users::UpdatableModel {
            email: Some("target@example.com".to_string()),
            username: Some("target_user".to_string()),
            ..Default::default()
        })
        .await;

    let result = users::Model::update_as_admin(
        &test_db.database,
        owner,
        target.clone(),
        users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        },
    )
    .await;
    assert!(result.is_ok());

    let updated = users::Entity::find_by_id(target.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(updated.roles, UserRoles::Admin);
}

#[test]
fn validate_name_chars_valid() {
    assert!(validate_name_chars("ValidUser").is_ok());
    assert!(validate_name_chars("user_123").is_ok());
    assert!(validate_name_chars("Jan Kowalski").is_ok());
    assert!(validate_name_chars("Łukasz").is_ok());
}

#[test]
fn validate_name_chars_invalid() {
    assert!(validate_name_chars("用户名").is_err());
    assert!(validate_name_chars("ユーザー").is_err());
}

#[test]
fn user_roles_permission_level() {
    assert_eq!(UserRoles::Owner.permission_level(), 2);
    assert_eq!(UserRoles::Admin.permission_level(), 1);
    assert_eq!(UserRoles::Default.permission_level(), 0);
}

#[test]
fn user_roles_ordering() {
    assert!(UserRoles::Owner > UserRoles::Admin);
    assert!(UserRoles::Admin > UserRoles::Default);
    assert!(UserRoles::Owner > UserRoles::Default);
    assert!(!(UserRoles::Default > UserRoles::Admin));
}

#[test]
fn password_debug_hides_value() {
    use hack4krak_backend::models::user::Password;
    let password = Password("secret".to_string());
    let debug_output = format!("{:?}", password);
    assert!(!debug_output.contains("secret"));
    assert!(debug_output.contains("******"));
}
