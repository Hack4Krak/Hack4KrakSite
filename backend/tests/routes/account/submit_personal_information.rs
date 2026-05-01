use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use serde::Deserialize;

#[derive(Deserialize)]
struct PersonalInfoResponse {
    first_name: String,
    location: String,
    ctf_experience: String,
    school_grade: String,
    collab_interest: bool,
    marketing_consent: bool,
    referral_sources: Option<serde_json::Value>,
}

async fn submit_personal_info_request(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Error = actix_web::Error,
    >,
    user: &hack4krak_backend::entities::users::Model,
    payload: serde_json::Value,
) -> actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody> {
    let request = test::TestRequest::post()
        .uri("/account/submit_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(payload)
        .to_request();
    test::call_service(app, request).await
}

async fn get_personal_info(
    app: &impl actix_web::dev::Service<
        actix_http::Request,
        Response = actix_web::dev::ServiceResponse<impl actix_web::body::MessageBody>,
        Error = actix_web::Error,
    >,
    user: &hack4krak_backend::entities::users::Model,
) -> PersonalInfoResponse {
    let request = test::TestRequest::get()
        .uri("/account/get_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    test::call_and_read_body_json(app, request).await
}

#[actix_web::test]
async fn account_delete() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/account/delete")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/account/")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn account_update() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/account/update")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
            "username": "Salieri",
        }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/account/")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_and_read_body(&app, request).await;
    assert_eq!(
        response,
        r#"{"username":"Salieri","email":"example@gmail.com","has_personal_information":false}"#
    );

    let request = test::TestRequest::patch()
        .uri("/account/update/password")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
            "old_password": "Dziengiel",
            "new_password": "Dziengiel2"
        }))
        .to_request();

    let response = test::call_service(&app, request).await;

    assert!(response.status().is_success());

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

#[actix_web::test]
async fn submit_personal_info() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/account/submit_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
          "first_name": "Antonio",
          "location": "Włochy",
          "ctf_experience": "Intermediate",
          "referral_sources": ["Linkedin"],
          "school_grade": "Class2",
          "marketing_consent": true,
          "collab_interest": false
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    let status = response.status();
    let body = test::read_body(response).await;
    assert!(
        status.is_success(),
        "status: {status}, body: {}",
        String::from_utf8_lossy(&body)
    );

    let request = test::TestRequest::get()
        .uri("/account/")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();

    let response = test::call_and_read_body(&app, request).await;
    assert_eq!(
        response,
        r#"{"username":"test_user","email":"example@gmail.com","has_personal_information":true}"#
    );

    let request = test::TestRequest::get()
        .uri("/account/get_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn submit_personal_info_invalid_referral_source() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/account/submit_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
          "first_name": "Antonio",
          "location": "Włochy",
          "ctf_experience": "Beginner",
          "referral_sources": ["Linkedin", "Invalid"],
          "school_grade": "Class2",
          "marketing_consent": true,
          "collab_interest": false
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn submit_personal_info_invalid_ctf_experience() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/account/submit_personal_information")
        .insert_header(TestAuthHeader::new(user.clone()))
        .set_json(serde_json::json!({
          "first_name": "Antonio",
          "location": "Włochy",
          "ctf_experience": "invalid_level",
          "referral_sources": ["Linkedin"],
          "school_grade": "Class2",
          "marketing_consent": true,
          "collab_interest": false
        }))
        .to_request();

    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn submit_personal_info_stores_correct_data() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;
    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let response = submit_personal_info_request(
        &app,
        &user,
        serde_json::json!({
            "first_name": "Katarzyna",
            "location": "Kraków",
            "ctf_experience": "Advanced",
            "school_grade": "University",
            "referral_sources": ["Discord", "Friend"],
            "marketing_consent": true,
            "collab_interest": true
        }),
    )
    .await;
    assert!(
        response.status().is_success(),
        "submit failed: {:?}",
        response.status()
    );

    let info = get_personal_info(&app, &user).await;

    assert_eq!(info.first_name, "Katarzyna");
    assert_eq!(info.location, "Kraków");
    assert_eq!(info.ctf_experience, "Advanced");
    assert_eq!(info.school_grade, "University");
    assert!(info.marketing_consent);
    assert!(info.collab_interest);

    let sources: Vec<String> =
        serde_json::from_value(info.referral_sources.unwrap()).unwrap();
    assert!(sources.contains(&"Discord".to_string()));
    assert!(sources.contains(&"Friend".to_string()));
    assert_eq!(sources.len(), 2);
}

#[actix_web::test]
async fn resubmit_updates_existing_data() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;
    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let response = submit_personal_info_request(
        &app,
        &user,
        serde_json::json!({
            "first_name": "Piotr",
            "location": "Warszawa",
            "ctf_experience": "Never",
            "school_grade": "Class1",
            "referral_sources": ["Instagram"],
            "marketing_consent": false,
            "collab_interest": false
        }),
    )
    .await;
    assert!(response.status().is_success());

    let response = submit_personal_info_request(
        &app,
        &user,
        serde_json::json!({
            "first_name": "Piotr",
            "location": "Kraków",
            "ctf_experience": "Expert",
            "school_grade": "University",
            "referral_sources": ["Discord"],
            "marketing_consent": true,
            "collab_interest": true
        }),
    )
    .await;
    assert!(response.status().is_success());

    let info = get_personal_info(&app, &user).await;

    assert_eq!(info.location, "Kraków");
    assert_eq!(info.ctf_experience, "Expert");
    assert_eq!(info.school_grade, "University");
    assert!(info.marketing_consent);
    assert!(info.collab_interest);

    let sources: Vec<String> =
        serde_json::from_value(info.referral_sources.unwrap()).unwrap();
    assert_eq!(sources, vec!["Discord"]);
}
