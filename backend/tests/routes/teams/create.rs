use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use hack4krak_backend::entities::users;
use serde_json::json;

#[actix_web::test]
async fn create_team_user_already_belongs_to_team() {
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

    let request = test::TestRequest::post()
        .uri("/teams/create")
        .set_json(json!({
            "team_name": "team1".to_string(),
        }))
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn create_duplicate_team() {
    let test_database = TestDatabase::new().await;
    test_database.with_default_team().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/teams/create")
        .set_json(json!({
            "team_name": "Dziengiel".to_string(),
        }))
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn create_team_success() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/teams/create")
        .set_json(json!({
            "team_name": "Dziengiel".to_string(),
        }))
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}
