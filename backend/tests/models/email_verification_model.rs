use crate::test_utils::database::TestDatabase;
use chrono::Utc;
use hack4krak_backend::entities::email_verification_request;
use sea_orm::{EntityTrait, Set};
use std::ops::{Add, Sub};
use uuid::Uuid;

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
