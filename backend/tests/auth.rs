use crate::utils::setup_test_app;
use actix_web::cookie::Cookie;
use actix_web::http::header;
use actix_web::test;
use hack4krak_backend::services::env::EnvConfig;
use utoipa::gen::serde_json::json;

mod utils;

#[actix_web::test]
async fn register() {
    let app = test::init_service(setup_test_app().await).await;

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
    let app = test::init_service(setup_test_app().await).await;

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

    let app = test::init_service(setup_test_app().await).await;

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

    let register_payload = json!({
        "email": "test@example.com",
        "password": "password123"
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
