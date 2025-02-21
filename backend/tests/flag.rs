use crate::utils::{init_database_with_teams, setup_test_app};
use actix_web::http::header;
use actix_web::test;
use actix_web::test::TestRequest;
use chrono::Duration;
use hack4krak_backend::models::task::TaskConfig;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::services::task_manager::TaskManager;
use hack4krak_backend::utils::jwt::encode_jwt;
use serde_json::json;

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

    let (database, _, _, users_uuid) = init_database_with_teams().await;

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
        users_uuid[0],
        "example@gmail.com".to_string(),
        Duration::minutes(10),
    )
    .unwrap();

    let request = submit_flag(&access_token, "hack4ka").await;
    let response: serde_json::Value  = test::call_and_read_body_json(&app, request.to_request()).await;
    assert_eq!(response["error"].as_str().unwrap(), "Flag(InvalidFlagFormat)");

    let request = submit_flag(&access_token, "hack4KrakCTF{...asds}").await;
    let response: serde_json::Value = test::call_and_read_body_json(&app, request.to_request()).await;
    assert_eq!(response["error"].as_str().unwrap(), "Flag(InvalidFlag)");

    let request = submit_flag(&access_token, "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 200);

    let request = submit_flag(&access_token, "hack4KrakCTF{skibidi}").await;
    let response = test::call_service(&app, request.to_request()).await;
    assert_eq!(response.status(), 409);
}
