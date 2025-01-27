use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{test, App};
use chrono::Duration;
use hack4krak_backend::models::entities::{teams, users};
use hack4krak_backend::routes;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::jwt::encode_jwt;
use sea_orm::{DatabaseBackend, MockDatabase};
use serde_json::json;
use std::env;
use utoipa_actix_web::scope;

#[actix_web::test]
async fn create_team_user_already_belongs_to_team() {
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            username: "Salieri".to_string(),
            email: "example@gmail.com".to_string(),
            created_at: Default::default(),
            team_name: Some("Dziengiel".to_string()),
            permissions: None,
            leads: None,
            password: None,
        }]])
        .append_query_results([vec![teams::Model {
            name: "Dziengiel".to_string(),
            created_at: Default::default(),
            leader_name: "Salieri".to_string(),
        }]])
        .append_exec_results([sea_orm::MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/teams").configure(routes::teams::config)),
    )
    .await;

    let access_token = encode_jwt("example@gmail.com".to_string(), Duration::minutes(10));

    let create_team_payload = json!({
        "team_name": "team1".to_string(),
    });

    let request = test::TestRequest::post()
        .uri("/teams/create_team")
        .set_json(&create_team_payload)
        .insert_header((
            header::AUTHORIZATION,
            format!("Bearer {}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn create_duplicate_team() {
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            username: "Salieri".to_string(),
            email: "example@gmail.com".to_string(),
            created_at: Default::default(),
            team_name: None,
            permissions: None,
            leads: None,
            password: None,
        }]])
        .append_query_results([vec![teams::Model {
            name: "Dziengiel".to_string(),
            created_at: Default::default(),
            leader_name: "".to_string(),
        }]])
        .append_exec_results([sea_orm::MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/teams").configure(routes::teams::config)),
    )
    .await;

    let create_team_payload = json!({
        "team_name": "Dziengiel".to_string(),
    });

    let access_token = encode_jwt("example@gmail.com".to_string(), Duration::minutes(10));

    let request = test::TestRequest::post()
        .uri("/teams/create_team")
        .set_json(&create_team_payload)
        .insert_header((
            header::AUTHORIZATION,
            format!("Bearer {}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn create_team_success() {
    let example_user = users::Model {
        username: "Salieri".to_string(),
        email: "example@gmail.com".to_string(),
        created_at: Default::default(),
        team_name: None,
        permissions: None,
        leads: None,
        password: None,
    };

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![example_user.clone()]])
        .append_query_results(vec![Vec::<teams::Model>::new()])
        .append_query_results(vec![vec![example_user.clone()]])
        .append_query_results(vec![vec![example_user]])
        .append_exec_results([sea_orm::MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .append_exec_results([sea_orm::MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/teams").configure(routes::teams::config)),
    )
    .await;

    let create_team_payload = json!({
        "team_name": "Dziengiel".to_string(),
    });

    let access_token = encode_jwt("example@gmail.com".to_string(), Duration::minutes(10));

    let request = test::TestRequest::post()
        .uri("/teams/create_team")
        .set_json(&create_team_payload)
        .insert_header((
            header::AUTHORIZATION,
            format!("Bearer {}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 200);
}
