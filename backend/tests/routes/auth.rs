use crate::test_utils::setup_test_app;

use crate::test_utils::mail::SmtpTestClient;
use actix_web::test::read_body_json;
use actix_web::web::Data;
use actix_web::{App, test};
use hack4krak_backend::entities::{email_confirmation, users};
use hack4krak_backend::routes;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::app_state::AppState;
use sea_orm::MockExecResult;
use sea_orm::{DatabaseBackend, MockDatabase};
use serde_json::json;
use utoipa_actix_web::scope;

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn register() {
    EnvConfig::load_test_config();

    let mail_client = SmtpTestClient::new().await;
    let app =
        test::init_service(setup_test_app(Some(mail_client.smtp_client.clone()), None, None).await)
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

    let first_email = &mail_client.get_emails().await.items[0];
    assert_eq!(first_email.raw.to[0].as_str(), "test@example.com");
}

#[actix_web::test]
async fn register_invalid_email() {
    let app = test::init_service(setup_test_app(None, None, None).await).await;

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
    // Verify if the body is proper JSON
    let _: serde_json::Value = read_body_json(response).await;
}

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn auth_flow() {
    EnvConfig::load_test_config();
    use actix_web::cookie::Cookie;
    use actix_web::http::header;

    const UUID_REGEX: &str =
        r"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}";

    let mail_client = SmtpTestClient::new().await;

    let app =
        test::init_service(setup_test_app(Some(mail_client.smtp_client.clone()), None, None).await)
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

    let emails = mail_client.get_emails().await;
    let email_body = &emails.items[0].content.body;

    let regex = regex::Regex::new(UUID_REGEX).unwrap();
    let confirmation_code = regex.find(email_body).unwrap().as_str();

    let request = test::TestRequest::get()
        .uri(&format!("/auth/confirm/{}", confirmation_code))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let login_payload = json!({
        "email": "test@example.com",
        "password": "password123"
    });

    let request = test::TestRequest::post()
        .uri("/auth/login")
        .set_json(&login_payload)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

    let access_token = response
        .headers()
        .get_all(header::SET_COOKIE)
        .map(|set_cookie| Cookie::parse(set_cookie.to_str().unwrap()).unwrap())
        .find(|cookie| cookie.name() == "access_token")
        .unwrap();

    let user_request = test::TestRequest::get()
        .uri("/account/")
        .insert_header((
            header::COOKIE,
            format!("access_token={}", access_token.value()),
        ))
        .to_request();

    let user_response = test::call_service(&app, user_request).await;

    assert!(user_response.status().is_success());
}

#[actix_web::test]
async fn email_confirmation_success() {
    EnvConfig::load_test_config();
    let confirmation_code = uuid::Uuid::new_v4().to_string();

    let email_confirmation = email_confirmation::Model {
        email: "".to_string(),
        user_info: json![{
                "name": "test_user",
                "email": "example@gmail.com",
                "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$nTzWdmrtGEOnwCocrg76xg$yv16FfDT5+meKwPmSiV+MF9kP8Man6bXZs+BloFTKIk".to_string(),
        }],
        code: confirmation_code.clone(),
        expiration_date: chrono::Local::now().naive_local() + chrono::Duration::minutes(30),
    };

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![email_confirmation.clone()],
        ])
        .append_query_results([
            Vec::<users::Model>::new(),
            vec![users::Model {
                id: Default::default(),
                username: "test_user".to_string(),
                email: "example@gmail.com".to_string(),
                created_at: Default::default(),
                team: None,
                is_leader: false,
                password: Some("$argon2id$v=19$m=19456,t=2,p=1$nTzWdmrtGEOnwCocrg76xg$yv16FfDT5+meKwPmSiV+MF9kP8Man6bXZs+BloFTKIk".to_string()),
                roles: Default::default(),
            }]
        ])
        .append_query_results([
            Vec::<email_confirmation::Model>::new(),
            vec![email_confirmation],
        ])
        .append_exec_results([MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
        .append_exec_results([MockExecResult {
            last_insert_id: 15,
            rows_affected: 1,
        }])
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

    let path = format!("/auth/confirm/{}", confirmation_code);

    let request = test::TestRequest::get().uri(&path).to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());
}

#[actix_web::test]
async fn email_confirmation_expired() {
    EnvConfig::load_test_config();

    let confirmation_code = uuid::Uuid::new_v4().to_string();

    let email_confirmation = email_confirmation::Model {
        email: "".to_string(),
        user_info: json![{
                "name": "test_user",
                "email": "example@gmail.com",
                "password_hash": "$argon2id$v=19$m=19456,t=2,p=1$nTzWdmrtGEOnwCocrg76xg$yv16FfDT5+meKwPmSiV+MF9kP8Man6bXZs+BloFTKIk".to_string(),
        }],
        code: confirmation_code.clone(),
        expiration_date: chrono::Local::now().naive_local(),
    };

    let database = MockDatabase::new(DatabaseBackend::Postgres)
        .append_query_results([
            vec![email_confirmation.clone()],
            Vec::<email_confirmation::Model>::new(),
            vec![email_confirmation],
        ])
        .into_connection();

    let app = test::init_service(
        App::new()
            .app_data(Data::new(AppState::with_database(database)))
            .service(scope("/auth").configure(routes::auth::config)),
    )
    .await;

    let path = format!("/auth/confirm/{}", confirmation_code);

    let request = test::TestRequest::get().uri(&path).to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 307);
}

#[cfg(feature = "full-test-suite")]
#[actix_web::test]
async fn reset_password_flow() {
    EnvConfig::load_test_config();
    use crate::test_utils::setup_database_with_schema;
    use chrono::Local;

    use sea_orm::{EntityTrait, Set};

    use uuid::Uuid;

    use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;

    const UUID_REGEX: &str =
        r"[a-fA-F0-9]{8}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{4}-[a-fA-F0-9]{12}";

    let mail_client = SmtpTestClient::new().await;

    let database = setup_database_with_schema().await;

    users::Entity::insert(users::ActiveModel {
        id: Set(Uuid::new_v4()),
        username: Set("test_user".to_string()),
        email: Set("example@gmail.com".to_string()),
        created_at: Set(Local::now().naive_local()),
        team: Set(None),
        is_leader: Set(false),
        password: Set(None),
        roles: Set(UserRoles::Default),
    })
    .exec(&database)
    .await
    .unwrap();

    let app = test::init_service(
        setup_test_app(Some(mail_client.smtp_client.clone()), Some(database), None).await,
    )
    .await;

    let payload = json!({
        "email": "example@gmail.com"
    });

    let request = test::TestRequest::post()
        .uri("/auth/request_reset_password")
        .set_json(payload)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 200);

    let email_body = &mail_client.get_emails().await.items[0].content.body;
    let regex = regex::Regex::new(UUID_REGEX).unwrap();
    let reset_code = regex.find(email_body).unwrap().as_str();

    let new_password = "meow123".to_string();

    let payload = json!({
        "code": reset_code.to_string(),
        "new_password": new_password.clone()
    });

    let request = test::TestRequest::patch()
        .uri("/auth/reset_password")
        .set_json(payload)
        .to_request();

    let response = test::call_service(&app, request).await;

    assert_eq!(response.status(), 200);
}
