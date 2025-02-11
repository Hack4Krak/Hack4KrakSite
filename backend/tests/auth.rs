use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::web::Data;
use actix_web::{test, App};
use hack4krak_backend::entities::users;
use hack4krak_backend::middlewares::auth::AuthMiddleware;
use hack4krak_backend::routes;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::app_state::AppState;
use sea_orm::{DatabaseBackend, MockDatabase, MockExecResult};
use utoipa::gen::serde_json::json;
use utoipa_actix_web::scope;

#[actix_web::test]
async fn register() {
    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            Vec::<users::Model>::new(),
            vec![users::Model {
                id: Default::default(),
                username: "".to_string(),
                email: "".to_string(),
                created_at: Default::default(),
                team: None,
                is_leader: false,
                password: None,
                roles: Default::default(),
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
    EnvConfig::load_test_config();

    let example_user = users::Model {
        id: Default::default(),
        username: "Developer".to_string(),
        email: "dev@hack4krak.eu".to_string(),
        created_at: Default::default(),
        team: None,
        is_leader: false,
        password: Some("$argon2id$v=19$m=19456,t=2,p=1$cLSl6N0HmRupZWoHO/b2EQ$rWWC3cagHlLCO2+awPqSHQCeypMtIM9GhHNqn1dzaik".to_string()),
        roles: Default::default(),
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
                    .wrap(AuthMiddleware::default())
                    .configure(routes::user::admin_config),
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

    let access_token = response
        .headers()
        .get_all(header::SET_COOKIE)
        .map(|set_cookie| Cookie::parse(set_cookie.to_str().unwrap()).unwrap())
        .find(|cookie| cookie.name() == "access_token")
        .unwrap();

    let user_request = test::TestRequest::get()
        .uri("/user/")
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.value()),
        ))
        .to_request();

    let user_response = test::call_service(&app, user_request).await;

    assert!(user_response.status().is_success());
}
