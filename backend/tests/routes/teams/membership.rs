use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use hack4krak_backend::entities::{team_invites, users};
use sea_orm::{ActiveModelTrait, Set};
use serde_json::Value;
use uuid::Uuid;

fn auth_header(user: &users::Model) -> TestAuthHeader {
    TestAuthHeader::new(user.id, user.email.clone())
}

#[actix_web::test]
async fn my_team_returns_null_for_user_without_team() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let body: Option<Value> = test::read_body_json(response).await;
    assert!(body.is_none());
}

#[actix_web::test]
async fn my_team_returns_invited_users_for_team_leader() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            username: Some("leader".to_string()),
            email: Some("leader@example.com".to_string()),
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    test_database
        .with_user(users::UpdatableModel {
            username: Some("member".to_string()),
            email: Some("member@example.com".to_string()),
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;
    let invited_user = test_database
        .with_user(users::UpdatableModel {
            username: Some("invited_user".to_string()),
            email: Some("invited@example.com".to_string()),
            ..Default::default()
        })
        .await;

    team_invites::ActiveModel {
        id: Set(Uuid::new_v4()),
        user: Set(invited_user.id),
        team: Set(team.id),
    }
    .insert(&test_database.database)
    .await
    .unwrap();

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(auth_header(&leader))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let body: Value = test::read_body_json(response).await;

    assert_eq!(body["team_name"], team.name);
    assert_eq!(body["invited_users"], serde_json::json!(["invited_user"]));
    assert_eq!(body["members"].as_array().unwrap().len(), 2);
}

#[actix_web::test]
async fn my_team_hides_invited_users_for_non_leader() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    test_database
        .with_user(users::UpdatableModel {
            username: Some("leader".to_string()),
            email: Some("leader@example.com".to_string()),
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let member = test_database
        .with_user(users::UpdatableModel {
            username: Some("member".to_string()),
            email: Some("member@example.com".to_string()),
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;
    let invited_user = test_database
        .with_user(users::UpdatableModel {
            username: Some("invited_user".to_string()),
            email: Some("invited@example.com".to_string()),
            ..Default::default()
        })
        .await;

    team_invites::ActiveModel {
        id: Set(Uuid::new_v4()),
        user: Set(invited_user.id),
        team: Set(team.id),
    }
    .insert(&test_database.database)
    .await
    .unwrap();

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(auth_header(&member))
        .to_request();
    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let body: Value = test::read_body_json(response).await;
    assert!(body["invited_users"].is_null());
}
