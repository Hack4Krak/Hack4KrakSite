use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::users;

#[actix_web::test]
async fn middleware_user_is_not_admin() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/account/admin")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn middleware_user_is_admin() {
    let test_database = TestDatabase::new().await;
    let user = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/account/admin")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}
