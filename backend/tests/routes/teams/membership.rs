use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use hack4krak_backend::entities::users;

#[actix_web::test]
async fn my_team_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn my_team_returns_members() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            username: Some("member".to_string()),
            email: Some("member@example.com".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response: serde_json::Value = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response["team_name"], "Dziengiel");
    assert_eq!(response["members"].as_array().unwrap().len(), 2);
}

#[actix_web::test]
async fn my_team_without_team_forbidden() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn leave_team_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let _leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            username: Some("leader".to_string()),
            email: Some("leader@example.com".to_string()),
            ..Default::default()
        })
        .await;
    let member = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/teams/membership/leave_team")
        .insert_header(TestAuthHeader::new(member))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn leave_team_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/teams/membership/leave_team")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn completed_tasks_empty() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/completed_tasks")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response: Vec<String> = test::call_and_read_body_json(&app, request).await;
    assert!(response.is_empty());
}

#[actix_web::test]
async fn stats_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/stats")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response: serde_json::Value = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response["captured_flags"], 0);
    assert_eq!(response["remaining_flags"], 0);
}

#[actix_web::test]
async fn stats_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/stats")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn completed_tasks_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/completed_tasks")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn leave_team_no_team() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/teams/membership/leave_team")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn my_team_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn stats_without_team() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/stats")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn completed_tasks_without_team() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/completed_tasks")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}
