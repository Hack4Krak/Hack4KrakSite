use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use actix_web::test;
use hack4krak_backend::entities::teams;
use uuid::Uuid;

#[actix_web::test]
async fn confirm_team_success() {
    let test_database = TestDatabase::new().await;

    let confirmation_code = Uuid::new_v4();
    test_database
        .with_team(teams::UpdatableModel {
            confirmation_code: Some(Some(confirmation_code)),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri(format!("/teams/confirm/{confirmation_code}").as_str())
        .method(actix_web::http::Method::POST)
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 200);
}

#[actix_web::test]
async fn confirm_team_invalid_confirmation_code() {
    let test_database = TestDatabase::new().await;
    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri(format!("/teams/confirm/{}", Uuid::new_v4()).as_str())
        .method(actix_web::http::Method::POST)
        .to_request();

    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}
