use crate::test_utils::{init_database_with_teams, setup_test_app};
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{App, test};
use chrono::{Duration, Local};
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::{teams, users};
use hack4krak_backend::routes;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::jwt::encode_jwt;
use sea_orm::ActiveValue::Set;
use sea_orm::{ActiveModelTrait, DatabaseBackend, MockDatabase};
use serde_json::json;
use utoipa_actix_web::scope;
use uuid::Uuid;

#[actix_web::test]
async fn create_team_user_already_belongs_to_team() {
    EnvConfig::load_test_config();

    let uuid = uuid::Uuid::new_v4();
    let team_id = uuid::Uuid::new_v4();

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            id: uuid,
            username: "Salieri".to_string(),
            email: "example@gmail.com".to_string(),
            created_at: Default::default(),
            team: Some(team_id),
            is_leader: false,
            password: None,
            roles: Default::default(),
        }]])
        .append_query_results([vec![teams::Model {
            id: team_id,
            name: "Dziengiel".to_string(),
            created_at: Default::default(),
            confirmation_code: Default::default(),
            status: Default::default(),
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

    let access_token = encode_jwt(uuid, "example@gmail.com".to_string(), Duration::minutes(10));

    let create_team_payload = json!({
        "team_name": "team1".to_string(),
    });

    let request = test::TestRequest::post()
        .uri("/teams/create")
        .set_json(&create_team_payload)
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn create_duplicate_team() {
    EnvConfig::load_test_config();

    let uuid = uuid::Uuid::new_v4();

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([vec![users::Model {
            id: uuid,
            username: "Salieri".to_string(),
            email: "example@gmail.com".to_string(),
            created_at: Default::default(),
            team: None,
            roles: Default::default(),
            is_leader: false,
            password: None,
        }]])
        .append_query_results([vec![teams::Model {
            id: Default::default(),
            name: "Dziengiel".to_string(),
            created_at: Default::default(),
            confirmation_code: Default::default(),
            status: Default::default(),
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

    let access_token = encode_jwt(uuid, "example@gmail.com".to_string(), Duration::minutes(10));

    let request = test::TestRequest::post()
        .uri("/teams/create")
        .set_json(&create_team_payload)
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn create_team_success() {
    EnvConfig::load_test_config();

    let uuid = uuid::Uuid::new_v4();

    let example_user = users::Model {
        id: uuid,
        username: "Salieri".to_string(),
        email: "example@gmail.com".to_string(),
        created_at: Default::default(),
        team: None,
        roles: Default::default(),
        is_leader: false,
        password: None,
    };

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![example_user.clone()]])
        .append_query_results(vec![Vec::<teams::Model>::new()])
        .append_query_results(vec![vec![example_user.clone()]])
        .append_query_results(vec![vec![example_user]])
        .append_query_results(vec![Vec::<teams::Model>::new()])
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

    let access_token = encode_jwt(uuid, "example@gmail.com".to_string(), Duration::minutes(10));

    let request = test::TestRequest::post()
        .uri("/teams/create")
        .set_json(&create_team_payload)
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.unwrap()),
        ))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 200);
}

#[actix_web::test]
async fn assert_correct_team_size() {
    EnvConfig::load_test_config();

    let (database, uuid, _, _) = init_database_with_teams().await;

    let app = test::init_service(setup_test_app(None, Some(database), None).await).await;

    let access_token =
        encode_jwt(uuid, "skibidi@gmail.com".to_string(), Duration::minutes(10)).unwrap();

    let request = test::TestRequest::post()
        .uri("/teams/invitations/accept_invitation/dziengiel")
        .insert_header((header::COOKIE, format!("access_token={}", access_token)))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 200);
}

#[actix_web::test]
async fn assert_incorrect_team_size() {
    EnvConfig::load_test_config();

    let (database, user_uuid, team_uuid, _) = init_database_with_teams().await;

    users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set("Salieri10".to_string()),
        email: Set("example123@gmail.com".to_string()),
        created_at: Set(Local::now().naive_local()),
        is_leader: Set(false),
        roles: Set(UserRoles::Default),
        team: Set(Some(team_uuid)),
        ..Default::default()
    }
    .insert(&database)
    .await
    .unwrap();

    let app = test::init_service(setup_test_app(None, Some(database), None).await).await;

    let access_token = encode_jwt(
        user_uuid,
        "skibidi@gmail.com".to_string(),
        Duration::minutes(10),
    )
    .unwrap();

    let request = test::TestRequest::post()
        .uri("/teams/invitations/accept_invitation/dziengiel")
        .insert_header((header::COOKIE, format!("access_token={}", access_token)))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn confirm_team_success() {
    EnvConfig::load_test_config();

    let confirmation_code = Uuid::new_v4();

    let team = teams::Model {
        id: Default::default(),
        name: "Dziengiel".to_string(),
        created_at: Default::default(),
        confirmation_code: Some(confirmation_code),
        status: Default::default(),
    };

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![team.clone()],
            vec![team.clone()],
            vec![team.clone()],
            vec![team],
        ])
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

    let request = test::TestRequest::get()
        .uri(format!("/teams/confirm/{}", confirmation_code).as_str())
        .method(actix_web::http::Method::POST)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 200);
}

#[actix_web::test]
async fn confirm_team_invalid_confirmation_code() {
    EnvConfig::load_test_config();

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([Vec::<teams::Model>::new()])
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

    let request = test::TestRequest::get()
        .uri(format!("/teams/confirm/{}", Uuid::new_v4()).as_str())
        .method(actix_web::http::Method::POST)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 400);
}
