use actix_web::body::MessageBody;
use actix_web::http::header;
use actix_web::middleware::from_fn;
use actix_web::web::Data;
use actix_web::{test, App};
use hack4krak_backend::models::entities::users;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::{middlewares, routes};
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use std::env;
use utoipa::gen::serde_json::json;
use utoipa_actix_web::scope;

#[actix_web::test]
async fn register() {
    env::set_var("JWT_SECRET", "secret");

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            Vec::<users::Model>::new(),
            vec![users::Model {
                username: "".to_string(),
                email: "".to_string(),
                created_at: Default::default(),
                team_name: None,
                permissions: None,
                leads: None,
                password: None,
            }],
        ])
        .append_exec_results([MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/auth").configure(routes::auth::config)),
    )
    .await;

    let register_payload = json!({
        "email": "test@example.com",
        "name": "test_user",
        "password": "password123"
    });

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&register_payload)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());
}

#[actix_web::test]
async fn register_invalid_email() {
    let database = MockDatabase::new(DatabaseBackend::Postgres).into_connection();
    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/auth").configure(routes::auth::config)),
    )
    .await;

    let register_payload = json!({
        "email": "this_!isn'taemaill",
        "name": "test_user",
        "password": "password123"
    });

    let request = test::TestRequest::post()
        .uri("/auth/register")
        .set_json(&register_payload)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn auth_flow() {
    let example_user = users::Model {
        username: "Developer".to_string(),
        email: "dev@hack4krak.eu".to_string(),
        created_at: Default::default(),
        team_name: None,
        permissions: None,
        leads: None,
        password: Some("$argon2id$v=19$m=19456,t=2,p=1$cLSl6N0HmRupZWoHO/b2EQ$rWWC3cagHlLCO2+awPqSHQCeypMtIM9GhHNqn1dzaik".to_string()),
    };

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results(vec![vec![example_user.clone()], vec![example_user]])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/auth").configure(routes::auth::config))
            .service(
                scope("/user")
                    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .configure(routes::user::config),
            ),
    )
    .await;

    let register_payload = json!({
        "email": "test@example.com",
        "password": "okok"
    });

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(&register_payload)
        .to_request();

    let response = test::call_service(&app, request).await;

    let body_bytes = response.into_body().try_into_bytes().unwrap();
    let body_str = String::from_utf8(body_bytes.to_vec()).unwrap();
    let parsed_response: serde_json::Value = serde_json::from_str(&body_str).unwrap();

    let access_token = parsed_response["access_token"].as_str().unwrap();
    let user_request = test::TestRequest::get()
        .uri("/user/")
        .insert_header((header::AUTHORIZATION, format!("Bearer {}", access_token)))
        .to_request();

    let user_response = test::call_service(&app, user_request).await;

    assert!(user_response.status().is_success());
}
