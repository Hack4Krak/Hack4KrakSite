use crate::utils::{setup_database_with_schema, setup_test_app};
use actix_web::http::header;
use actix_web::test;
use actix_web::test::TestRequest;
use chrono::{Duration, Local};
use hack4krak_backend::entities::sea_orm_active_enums::{TeamStatus, UserRoles};
use hack4krak_backend::entities::{teams, users};
use hack4krak_backend::models::task::TaskConfig;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::services::task_manager::TaskManager;
use hack4krak_backend::utils::jwt::encode_jwt;
use sea_orm::ActiveModelTrait;
use sea_orm::ActiveValue::Set;
use serde_json::json;
use uuid::Uuid;

mod utils;

async fn submit_flag(access_token: &str, flag: &str) -> TestRequest {
    TestRequest::post()
        .uri("/flag/submit")
        .insert_header((header::COOKIE, format!("access_token={}", access_token)))
        .set_json(json!({ "flag": flag }))
}

#[actix_web::test]
async fn try_submitting_flags() {
    EnvConfig::load_test_config();

    let database = setup_database_with_schema().await;

    let team_uuid = Uuid::new_v4();

    teams::ActiveModel {
        id: Set(team_uuid),
        name: Set("dziengiel".to_string()),
        created_at: Set(Local::now().naive_local()),
        confirmation_code: Set(Some(team_uuid)),
        status: Set(TeamStatus::Confirmed),
    }
    .insert(&database)
    .await
    .unwrap();

    let team_uuid2 = Uuid::new_v4();

    teams::ActiveModel {
        id: Set(team_uuid2),
        name: Set("dziengiel2".to_string()),
        created_at: Set(Local::now().naive_local()),
        confirmation_code: Set(Some(Uuid::new_v4())),
        status: Set(TeamStatus::Absent),
    }
    .insert(&database)
    .await
    .unwrap();

    let user_from_confirmed_team_uuid = Uuid::new_v4();

    users::ActiveModel {
        id: Set(user_from_confirmed_team_uuid),
        username: Set("Antonio".to_string()),
        email: Set("skibidi@gmail.com".to_string()),
        created_at: Set(Local::now().naive_local()),
        team: Set(Some(team_uuid)),
        is_leader: Set(false),
        roles: Set(UserRoles::Default),
        ..Default::default()
    }
    .insert(&database)
    .await
    .unwrap();

    let user_from_absent_team_uuid = Uuid::new_v4();

    users::ActiveModel {
        id: Set(user_from_absent_team_uuid),
        username: Set("Antonio2".to_string()),
        email: Set("skibidi2@gmail.com".to_string()),
        created_at: Set(Local::now().naive_local()),
        team: Set(Some(team_uuid2)),
        is_leader: Set(false),
        roles: Set(UserRoles::Default),
        ..Default::default()
    }
    .insert(&database)
    .await
    .unwrap();

    let task_manager = TaskManager::default();
    task_manager.tasks.insert(
        "test-task".to_string(),
        TaskConfig {
            flag_hash: "1912766d6ba0e50e8b1bacfb51207e83b95b7ac0cd8ce15307cdf4965e7e3f6c"
                .to_string(),
            ..Default::default()
        },
    );

    let app =
        test::init_service(setup_test_app(None, Some(database), Some(task_manager)).await).await;

    let access_token = encode_jwt(
        user_from_absent_team_uuid,
        "skibidi2@gmail.com".to_string(),
        Duration::minutes(10),
    )
    .unwrap();

    let request = submit_flag(&access_token, "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 403);

    let access_token = encode_jwt(
        user_from_confirmed_team_uuid,
        "skibidi@gmail.com".to_string(),
        Duration::minutes(10),
    )
    .unwrap();

    let request = submit_flag(&access_token, "hack4ka").await;
    let response: serde_json::Value =
        test::call_and_read_body_json(&app, request.to_request()).await;
    assert_eq!(
        response["error"].as_str().unwrap(),
        "Flag(InvalidFlagFormat)"
    );

    let request = submit_flag(&access_token, "hack4KrakCTF{...asds}").await;
    let response: serde_json::Value =
        test::call_and_read_body_json(&app, request.to_request()).await;
    assert_eq!(response["error"].as_str().unwrap(), "Flag(InvalidFlag)");

    let request = submit_flag(&access_token, "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 200);

    let request = submit_flag(&access_token, "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 409);
}
