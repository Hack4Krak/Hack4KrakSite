use crate::test_utils;
use crate::test_utils::setup_test_app;
use actix_web::http::header;
use actix_web::test;
use chrono::Duration;
use hack4krak_backend::entities::users;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::jwt::encode_jwt;
use sea_orm::EntityTrait;

#[actix_web::test]
async fn account_delete() {
    EnvConfig::load_test_config();

    let (database, user_uuid) = test_utils::init_database_with_user().await;

    let user = users::Entity::find_by_id(user_uuid)
        .one(&database)
        .await
        .unwrap()
        .unwrap();

    let app = test::init_service(setup_test_app(None, Some(database), None).await).await;

    let access_token = encode_jwt(user.id, user.email, Duration::minutes(10)).unwrap();

    let request = test::TestRequest::delete()
        .uri("/account/delete")
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.clone()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/account/")
        .insert_header((header::COOKIE, format!("access_token={}", access_token)))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn account_update() {
    EnvConfig::load_test_config();

    let (database, user_uuid) = test_utils::init_database_with_user().await;

    let user = users::Entity::find_by_id(user_uuid)
        .one(&database)
        .await
        .unwrap()
        .unwrap();

    let app = test::init_service(setup_test_app(None, Some(database), None).await).await;

    let access_token = encode_jwt(user.id, user.email, Duration::minutes(10)).unwrap();

    let request = test::TestRequest::patch()
        .uri("/account/update")
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.clone()),
        ))
        .set_json(serde_json::json!({
            "username": "Salieri",
            "old_password": "Dziengiel",
            "new_password": "Dziengiel2"
        }))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/account/")
        .insert_header((header::COOKIE, format!("access_token={}", access_token)))
        .to_request();

    let response = test::call_and_read_body(&app, request).await;

    assert_eq!(
        response,
        r#"{"username":"Salieri","email":"example@gmail.com"}"#
    );

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(serde_json::json!({
            "email": "example@gmail.com",
            "password": "Dziengiel2"
        }))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());
}
