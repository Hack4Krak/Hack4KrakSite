use chrono::Utc;
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::users;
use hack4krak_backend::models::user::Password;
use hack4krak_backend::services::auth::AuthService;
use uuid::Uuid;

#[test]
fn hash_password_returns_valid_argon2_hash() {
    let password = Password("test_password123".to_string());
    let hash = AuthService::hash_password(password).unwrap();
    assert!(hash.starts_with("$argon2"));
}

#[test]
fn hash_password_produces_different_hashes_for_same_input() {
    let hash1 = AuthService::hash_password(Password("same_password".to_string())).unwrap();
    let hash2 = AuthService::hash_password(Password("same_password".to_string())).unwrap();
    assert_ne!(
        hash1, hash2,
        "Different salts should produce different hashes"
    );
}

#[test]
fn hash_password_works_with_empty_password() {
    let result = AuthService::hash_password(Password(String::new()));
    assert!(result.is_ok());
}

#[test]
fn hash_password_works_with_long_password() {
    let long_password = "a".repeat(256);
    let result = AuthService::hash_password(Password(long_password));
    assert!(result.is_ok());
}

#[test]
fn assert_password_is_valid_succeeds_with_correct_password() {
    let password = Password("correct_password".to_string());
    let hash = AuthService::hash_password(password.clone()).unwrap();
    let user = users::Model {
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        created_at: Utc::now().naive_utc(),
        password: Some(hash),
        id: Uuid::new_v4(),
        is_leader: false,
        team: None,
        roles: UserRoles::User,
        personal_info: None,
    };

    let result =
        AuthService::assert_password_is_valid(&user, &Password("correct_password".to_string()));
    assert!(result.is_ok());
}

#[test]
fn assert_password_is_valid_fails_with_wrong_password() {
    let password = Password("correct_password".to_string());
    let hash = AuthService::hash_password(password).unwrap();
    let user = users::Model {
        username: "test".to_string(),
        email: "test@example.com".to_string(),
        created_at: Utc::now().naive_utc(),
        password: Some(hash),
        id: Uuid::new_v4(),
        is_leader: false,
        team: None,
        roles: UserRoles::User,
        personal_info: None,
    };

    let result =
        AuthService::assert_password_is_valid(&user, &Password("wrong_password".to_string()));
    assert!(result.is_err());
}

#[test]
fn assert_password_is_valid_fails_when_user_has_no_password() {
    let user = users::Model {
        username: "oauth_user".to_string(),
        email: "oauth@example.com".to_string(),
        created_at: Utc::now().naive_utc(),
        password: None,
        id: Uuid::new_v4(),
        is_leader: false,
        team: None,
        roles: UserRoles::User,
        personal_info: None,
    };

    let result =
        AuthService::assert_password_is_valid(&user, &Password("any_password".to_string()));
    assert!(result.is_err());
}
