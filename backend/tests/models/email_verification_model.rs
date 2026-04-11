use crate::test_utils::database::TestDatabase;
use chrono::{Duration, Utc};
use hack4krak_backend::entities::email_verification_request;
use hack4krak_backend::models::email_verification_request::EmailVerificationAction;
use hack4krak_backend::models::user::UserInformation;
use sea_orm::{EntityTrait, Set};
use std::ops::{Add, Sub};
use uuid::Uuid;

#[actix_web::test]
async fn create_stores_request_in_database() {
    let test_db = TestDatabase::new().await;

    let action = EmailVerificationAction::ResetPassword;
    let email = "test@example.com".to_string();

    let code = email_verification_request::Model::create(
        &test_db.database,
        action,
        email.clone(),
        Some(Duration::minutes(30)),
    )
    .await
    .unwrap();

    let stored = email_verification_request::Entity::find_by_id(code)
        .one(&test_db.database)
        .await
        .unwrap();
    assert!(stored.is_some());

    let stored = stored.unwrap();
    assert_eq!(stored.email, email);
    assert_eq!(stored.action_type, "reset_password");
    assert!(stored.expiration_time.is_some());
}

#[actix_web::test]
async fn create_without_expiration() {
    let test_db = TestDatabase::new().await;

    let action = EmailVerificationAction::ResetPassword;

    let code = email_verification_request::Model::create(
        &test_db.database,
        action,
        "test@example.com".to_string(),
        None,
    )
    .await
    .unwrap();

    let stored = email_verification_request::Entity::find_by_id(code)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(stored.expiration_time.is_none());
}

#[actix_web::test]
async fn create_with_confirm_email_action_stores_user_info() {
    let test_db = TestDatabase::new().await;

    let action = EmailVerificationAction::ConfirmEmailAddress {
        user_information: UserInformation {
            name: "testuser".to_string(),
            email: "test@example.com".to_string(),
            password_hash: "hashed".to_string(),
        },
    };

    let code = email_verification_request::Model::create(
        &test_db.database,
        action,
        "test@example.com".to_string(),
        Some(Duration::minutes(30)),
    )
    .await
    .unwrap();

    let stored = email_verification_request::Entity::find_by_id(code)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(stored.action_type, "confirm_email_address");
    assert!(stored.additional_data.is_some());

    let action = stored.get_action().unwrap();
    match action {
        EmailVerificationAction::ConfirmEmailAddress { user_information } => {
            assert_eq!(user_information.name, "testuser");
            assert_eq!(user_information.email, "test@example.com");
        }
        _ => panic!("Expected ConfirmEmailAddress variant"),
    }
}

#[actix_web::test]
async fn find_and_verify_valid_request() {
    let test_db = TestDatabase::new().await;

    let code = email_verification_request::Model::create(
        &test_db.database,
        EmailVerificationAction::ResetPassword,
        "test@example.com".to_string(),
        Some(Duration::minutes(30)),
    )
    .await
    .unwrap();

    let result = email_verification_request::Model::find_and_verify(&test_db.database, code).await;
    assert!(result.is_ok());
    assert_eq!(result.unwrap().email, "test@example.com");
}

#[actix_web::test]
async fn find_and_verify_expired_request_returns_error_and_deletes() {
    let test_db = TestDatabase::new().await;

    let expired_id = Uuid::new_v4();
    email_verification_request::Entity::insert(email_verification_request::ActiveModel {
        id: Set(expired_id),
        action_type: Set("reset_password".to_string()),
        additional_data: Set(None),
        email: Set("test@example.com".to_string()),
        expiration_time: Set(Some(Utc::now().naive_utc().sub(Duration::hours(1)))),
        created_at: Set(Utc::now().naive_utc()),
    })
    .exec(&test_db.database)
    .await
    .unwrap();

    let result =
        email_verification_request::Model::find_and_verify(&test_db.database, expired_id).await;
    assert!(result.is_err());

    let deleted = email_verification_request::Entity::find_by_id(expired_id)
        .one(&test_db.database)
        .await
        .unwrap();
    assert!(
        deleted.is_none(),
        "Expired request should be deleted after verification attempt"
    );
}

#[actix_web::test]
async fn find_and_verify_nonexistent_code_returns_error() {
    let test_db = TestDatabase::new().await;

    let result =
        email_verification_request::Model::find_and_verify(&test_db.database, Uuid::new_v4()).await;
    assert!(result.is_err());
}

#[actix_web::test]
async fn find_and_verify_without_expiration_succeeds() {
    let test_db = TestDatabase::new().await;

    let code = email_verification_request::Model::create(
        &test_db.database,
        EmailVerificationAction::ResetPassword,
        "test@example.com".to_string(),
        None,
    )
    .await
    .unwrap();

    let result = email_verification_request::Model::find_and_verify(&test_db.database, code).await;
    assert!(result.is_ok());
}

#[actix_web::test]
async fn delete_removes_request_from_database() {
    let test_db = TestDatabase::new().await;

    let code = email_verification_request::Model::create(
        &test_db.database,
        EmailVerificationAction::ResetPassword,
        "test@example.com".to_string(),
        None,
    )
    .await
    .unwrap();

    let request = email_verification_request::Model::find_and_verify(&test_db.database, code)
        .await
        .unwrap();

    request.delete(&test_db.database).await.unwrap();

    let deleted = email_verification_request::Entity::find_by_id(code)
        .one(&test_db.database)
        .await
        .unwrap();
    assert!(deleted.is_none());
}

#[actix_web::test]
async fn get_action_reset_password() {
    let test_db = TestDatabase::new().await;

    let code = email_verification_request::Model::create(
        &test_db.database,
        EmailVerificationAction::ResetPassword,
        "test@example.com".to_string(),
        None,
    )
    .await
    .unwrap();

    let request = email_verification_request::Model::find_and_verify(&test_db.database, code)
        .await
        .unwrap();

    let action = request.get_action().unwrap();
    assert!(matches!(action, EmailVerificationAction::ResetPassword));
}

#[actix_web::test]
async fn get_action_register_team() {
    let test_db = TestDatabase::new().await;

    let action = EmailVerificationAction::RegisterTeam {
        organization: "AGH".to_string(),
    };

    let code = email_verification_request::Model::create(
        &test_db.database,
        action,
        "test@example.com".to_string(),
        None,
    )
    .await
    .unwrap();

    let request = email_verification_request::Model::find_and_verify(&test_db.database, code)
        .await
        .unwrap();

    let restored_action = request.get_action().unwrap();
    match restored_action {
        EmailVerificationAction::RegisterTeam { organization } => {
            assert_eq!(organization, "AGH");
        }
        _ => panic!("Expected RegisterTeam variant"),
    }
}

#[actix_web::test]
async fn delete_expired_email_verification_requests() {
    let database = TestDatabase::new().await;
    let id_expired = Uuid::new_v4();

    email_verification_request::Entity::insert(email_verification_request::ActiveModel {
        id: Set(id_expired),
        action_type: Set("confirm_email_address".to_string()),
        additional_data: Set(None),
        email: Set("example@gmail.com".to_string()),
        expiration_time: Set(Some(Utc::now().naive_utc().sub(chrono::Duration::days(1)))),
        created_at: Set(Utc::now().naive_utc()),
    })
    .exec(&database.database)
    .await
    .unwrap();

    let id_not_expired = Uuid::new_v4();

    email_verification_request::Entity::insert(email_verification_request::ActiveModel {
        id: Set(id_not_expired),
        action_type: Set("confirm_email_address".to_string()),
        additional_data: Set(None),
        email: Set("example@gmail.com".to_string()),
        expiration_time: Set(Some(Utc::now().naive_utc().add(chrono::Duration::days(1)))),
        created_at: Set(Utc::now().naive_utc()),
    })
    .exec(&database.database)
    .await
    .unwrap();

    let id_without_expiration = Uuid::new_v4();

    email_verification_request::Entity::insert(email_verification_request::ActiveModel {
        id: Set(id_without_expiration),
        action_type: Set("confirm_email_address".to_string()),
        additional_data: Set(None),
        email: Set("example@gmail.com".to_string()),
        expiration_time: Set(None),
        created_at: Set(Utc::now().naive_utc()),
    })
    .exec(&database.database)
    .await
    .unwrap();

    email_verification_request::Model::delete_expired(&database.database)
        .await
        .unwrap();

    let is_expired_deleted = email_verification_request::Entity::find_by_id(id_expired)
        .one(&database.database)
        .await
        .unwrap()
        .is_none();

    assert!(
        is_expired_deleted,
        "The expired email verification request should be deleted"
    );

    let is_not_expired_still_present =
        email_verification_request::Entity::find_by_id(id_not_expired)
            .one(&database.database)
            .await
            .unwrap()
            .is_some();

    assert!(
        is_not_expired_still_present,
        "The not expired email verification request should still be present"
    );

    let is_without_expiration_still_present =
        email_verification_request::Entity::find_by_id(id_without_expiration)
            .one(&database.database)
            .await
            .unwrap()
            .is_some();

    assert!(
        is_without_expiration_still_present,
        "The email verification request without expiration should still be present"
    );
}
