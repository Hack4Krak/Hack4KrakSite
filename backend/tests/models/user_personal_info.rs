use crate::test_utils::database::TestDatabase;
use hack4krak_backend::entities::{user_personal_info, users};
use hack4krak_backend::routes::account::UserPersonalInformationSubmissionRequest;
use sea_orm::EntityTrait;

fn default_submission() -> UserPersonalInformationSubmissionRequest {
    UserPersonalInformationSubmissionRequest {
        first_name: "Jan".to_string(),
        birth_year: 2000,
        location: "Kraków".to_string(),
        organization: "AGH".to_string(),
        is_vegetarian: false,
        marketing_consent: true,
        referral_source: None,
    }
}

#[tokio::test]
async fn create_personal_info_for_new_user() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;
    assert!(user.personal_info.is_none());

    user_personal_info::Model::create(&test_db.database, user.clone(), default_submission())
        .await
        .unwrap();

    let updated_user = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert!(updated_user.personal_info.is_some());

    let info = user_personal_info::Entity::find_by_id(updated_user.personal_info.unwrap())
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    assert_eq!(info.first_name, "Jan");
    assert_eq!(info.birth_year, 2000);
    assert_eq!(info.location, "Kraków");
    assert_eq!(info.organization, "AGH");
    assert!(!info.is_vegetarian);
    assert!(info.marketing_consent);
}

#[tokio::test]
async fn create_personal_info_with_referral_source() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let submission = UserPersonalInformationSubmissionRequest {
        referral_source: Some(vec!["Facebook".to_string(), "Znajomy".to_string()]),
        ..default_submission()
    };

    user_personal_info::Model::create(&test_db.database, user.clone(), submission)
        .await
        .unwrap();

    let updated_user = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    let info = user_personal_info::Entity::find_by_id(updated_user.personal_info.unwrap())
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();

    assert!(info.referral_source.is_some());
    let sources: Vec<String> = serde_json::from_value(info.referral_source.unwrap()).unwrap();
    assert_eq!(sources.len(), 2);
    assert!(sources.contains(&"Facebook".to_string()));
    assert!(sources.contains(&"Znajomy".to_string()));
}

#[tokio::test]
async fn update_existing_personal_info() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    user_personal_info::Model::create(&test_db.database, user.clone(), default_submission())
        .await
        .unwrap();

    let updated_user = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    let personal_info_id = updated_user.personal_info.unwrap();

    let updated_submission = UserPersonalInformationSubmissionRequest {
        first_name: "Paweł".to_string(),
        birth_year: 1995,
        is_vegetarian: true,
        ..default_submission()
    };

    user_personal_info::Model::create(&test_db.database, updated_user, updated_submission)
        .await
        .unwrap();

    let info = user_personal_info::Entity::find_by_id(personal_info_id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(info.first_name, "Paweł");
    assert_eq!(info.birth_year, 1995);
    assert!(info.is_vegetarian);
    assert_eq!(
        info.id, personal_info_id,
        "Should update in place, not create new row"
    );
}

#[tokio::test]
async fn update_preserves_personal_info_id_on_user() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    user_personal_info::Model::create(&test_db.database, user.clone(), default_submission())
        .await
        .unwrap();

    let after_first = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();

    user_personal_info::Model::create(&test_db.database, after_first.clone(), default_submission())
        .await
        .unwrap();

    let after_second = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();

    assert_eq!(after_first.personal_info, after_second.personal_info);
}

#[tokio::test]
async fn create_with_vegetarian_flag_set() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let submission = UserPersonalInformationSubmissionRequest {
        is_vegetarian: true,
        ..default_submission()
    };

    user_personal_info::Model::create(&test_db.database, user.clone(), submission)
        .await
        .unwrap();

    let updated_user = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    let info = user_personal_info::Entity::find_by_id(updated_user.personal_info.unwrap())
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();

    assert!(info.is_vegetarian);
}

#[tokio::test]
async fn create_without_marketing_consent() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let submission = UserPersonalInformationSubmissionRequest {
        marketing_consent: false,
        ..default_submission()
    };

    user_personal_info::Model::create(&test_db.database, user.clone(), submission)
        .await
        .unwrap();

    let updated_user = users::Entity::find_by_id(user.id)
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();
    let info = user_personal_info::Entity::find_by_id(updated_user.personal_info.unwrap())
        .one(&test_db.database)
        .await
        .unwrap()
        .unwrap();

    assert!(!info.marketing_consent);
}
