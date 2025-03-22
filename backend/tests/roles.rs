use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{App, test};
use chrono::Duration;
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::users;
use hack4krak_backend::routes;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::jwt::encode_jwt;
use sea_orm::{DatabaseBackend, MockDatabase};
use utoipa_actix_web::scope;

#[actix_web::test]
async fn middleware_user_is_not_admin() {
    EnvConfig::load_test_config();

    let uuid = uuid::Uuid::new_v4();

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            id: uuid,
            username: "".to_string(),
            email: "example@gmail.com".to_string(),
            created_at: Default::default(),
            team: None,
            is_leader: false,
            password: None,
            roles: Default::default(),
        }]])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/account").configure(routes::account::config)),
    )
    .await;

    let access_token = encode_jwt(uuid, "example@gmail.com".to_string(), Duration::minutes(10));

    let request = test::TestRequest::get()
        .uri("/account/admin")
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn middleware_user_is_admin() {
    EnvConfig::load_test_config();

    let uuid = uuid::Uuid::new_v4();

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            id: uuid,
            username: "".to_string(),
            email: "example@gmail.com".to_string(),
            created_at: Default::default(),
            team: None,
            is_leader: false,
            password: None,
            roles: UserRoles::Admin,
        }]])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/account").configure(routes::account::config)),
    )
    .await;

    let access_token = encode_jwt(uuid, "example@gmail.com".to_string(), Duration::minutes(10));

    let request = test::TestRequest::get()
        .uri("/account/admin")
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());
}
