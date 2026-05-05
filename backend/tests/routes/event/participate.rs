use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use chrono::{DateTime, Duration, Utc};
use hack4krak_backend::entities::users;
use hack4krak_backend::services::task_manager::TaskManager;
use serde_json::{Value, json};

fn valid_payload() -> serde_json::Value {
    json!({
        "full_name": "Jan Kowalski",
        "school": "XIV Liceum Ogólnokształcące w Krakowie",
        "birth_year": "2007",
        "phone": "+48123456789",
        "is_underage": false,
        "emergency_contact_name": null,
        "emergency_contact_phone": null,
        "emergency_contact_email": null,
        "food_preference": "standard",
        "food_allergies": null,
    })
}

fn auth_header(user: &users::Model) -> TestAuthHeader {
    TestAuthHeader::new(user.id, user.email.clone())
}

#[actix_web::test]
async fn participate_success() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(valid_payload())
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response: Value = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response["full_name"].as_str().unwrap(), "Jan Kowalski");
    assert_eq!(response["food_preference"].as_str().unwrap(), "standard");
}

#[actix_web::test]
async fn participate_already_registered() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(valid_payload())
        .insert_header(auth_header(&user))
        .to_request();
    test::call_service(&app, request).await;

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(valid_payload())
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn participate_registration_closed() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let task_manager = TaskManager::default();
    task_manager.registration_config.write().await.end_date =
        DateTime::from(Utc::now() - Duration::minutes(10));

    let app = TestApp::default()
        .with_database(test_database)
        .with_task_manager(task_manager)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(valid_payload())
        .insert_header(auth_header(&user))
        .to_request();
    let response: Value = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response["error"].as_str().unwrap(), "Event");
}

#[actix_web::test]
async fn get_participation_not_registered() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn participate_unauthenticated() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(valid_payload())
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn participate_with_underage_and_emergency_contact() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let payload = json!({
        "full_name": "Jan Kowalski",
        "school": "XIV Liceum w Krakowie",
        "birth_year": "2009",
        "phone": "+48123456789",
        "is_underage": true,
        "emergency_contact_name": "Anna Kowalska",
        "emergency_contact_phone": "+48987654321",
        "emergency_contact_email": "anna@example.com",
        "food_preference": "vegetarian",
        "food_allergies": "orzechy",
    });

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(payload)
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response: Value = test::call_and_read_body_json(&app, request).await;
    assert!(response["is_underage"].as_bool().unwrap());
    assert_eq!(
        response["emergency_contact_name"].as_str().unwrap(),
        "Anna Kowalska"
    );
    assert_eq!(response["food_preference"].as_str().unwrap(), "vegetarian");
}

#[actix_web::test]
async fn delete_participation_success() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(valid_payload())
        .insert_header(auth_header(&user))
        .to_request();
    test::call_service(&app, request).await;

    let request = test::TestRequest::delete()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());

    let request = test::TestRequest::get()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn delete_participation_rejects_team_leader() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(valid_payload())
        .insert_header(auth_header(&user))
        .to_request();
    test::call_service(&app, request).await;

    let request = test::TestRequest::post()
        .uri("/teams/create")
        .set_json(json!({
            "team_name": "Unregistering Leaders",
        }))
        .insert_header(auth_header(&user))
        .to_request();
    test::call_service(&app, request).await;

    let request = test::TestRequest::delete()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 409);

    let request = test::TestRequest::get()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn delete_participation_rejects_team_member() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/event/participate")
        .set_json(valid_payload())
        .insert_header(auth_header(&user))
        .to_request();
    test::call_service(&app, request).await;

    let request = test::TestRequest::delete()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 409);

    let request = test::TestRequest::get()
        .uri("/event/participate")
        .insert_header(auth_header(&user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}
