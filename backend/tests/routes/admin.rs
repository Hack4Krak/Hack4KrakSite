use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use hack4krak_backend::entities::sea_orm_active_enums::UserRoles;
use hack4krak_backend::entities::{teams, users};
use serde_json::{Value, json};
use uuid::Uuid;

#[actix_web::test]
async fn admin_index_as_admin() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/admin/")
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn admin_index_as_default_user_forbidden() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/admin/")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn admin_users_list() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    test_db
        .with_user(users::UpdatableModel {
            email: Some("user2@example.com".to_string()),
            username: Some("user2".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/admin/users/list")
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response.len(), 2);
}

#[actix_web::test]
async fn admin_users_delete_success() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    let target = test_db
        .with_user(users::UpdatableModel {
            email: Some("target@example.com".to_string()),
            username: Some("target_user".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::delete()
        .uri(&format!("/admin/users/delete/{}", target.id))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn admin_users_delete_insufficient_role() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    let owner = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Owner),
            email: Some("owner@example.com".to_string()),
            username: Some("owner_user".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::delete()
        .uri(&format!("/admin/users/delete/{}", owner.id))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn admin_users_delete_not_found() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::delete()
        .uri(&format!("/admin/users/delete/{}", Uuid::new_v4()))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn admin_users_update_success() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    let target = test_db
        .with_user(users::UpdatableModel {
            email: Some("target@example.com".to_string()),
            username: Some("target_user".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::patch()
        .uri(&format!("/admin/users/update/{}", target.id))
        .insert_header(TestAuthHeader::new(admin))
        .set_json(json!({ "username": "new_name" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn admin_users_update_not_found() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::patch()
        .uri(&format!("/admin/users/update/{}", Uuid::new_v4()))
        .insert_header(TestAuthHeader::new(admin))
        .set_json(json!({ "username": "new_name" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn admin_email_confirmations_list() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/admin/users/email_confirmations")
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert!(response.is_empty());
}

#[actix_web::test]
async fn admin_teams_list() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    test_db.with_default_team().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/admin/teams/list")
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response: Vec<Value> = test::call_and_read_body_json(&app, request).await;
    assert_eq!(response.len(), 1);
}

#[actix_web::test]
async fn admin_teams_delete_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::delete()
        .uri(&format!("/admin/teams/delete/{}", team.id))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn admin_teams_update_success() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::patch()
        .uri(&format!("/admin/teams/update/{}", team.id))
        .insert_header(TestAuthHeader::new(admin))
        .set_json(json!({ "name": "UpdatedTeam" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn admin_teams_update_invalid_color() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::patch()
        .uri(&format!("/admin/teams/update/{}", team.id))
        .insert_header(TestAuthHeader::new(admin))
        .set_json(json!({ "color": "not-a-color" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn admin_teams_update_not_found() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::patch()
        .uri(&format!("/admin/teams/update/{}", Uuid::new_v4()))
        .insert_header(TestAuthHeader::new(admin))
        .set_json(json!({ "name": "UpdatedTeam" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn admin_teams_generate_confirmation_code() {
    let test_db = TestDatabase::new().await;
    let team = test_db.with_default_team().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::patch()
        .uri(&format!(
            "/admin/teams/generate_confirmation_code/{}",
            team.id
        ))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn admin_teams_generate_confirmation_code_not_found() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::patch()
        .uri(&format!(
            "/admin/teams/generate_confirmation_code/{}",
            Uuid::new_v4()
        ))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn admin_teams_clear_confirmation_code() {
    let test_db = TestDatabase::new().await;
    let code = Uuid::new_v4();
    let team = test_db
        .with_team(teams::UpdatableModel {
            confirmation_code: Some(Some(code)),
            ..Default::default()
        })
        .await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::delete()
        .uri(&format!("/admin/teams/clear_confirmation_code/{}", team.id))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn admin_get_personal_information_no_info() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;
    let target = test_db
        .with_user(users::UpdatableModel {
            email: Some("target@example.com".to_string()),
            username: Some("target_user".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri(&format!(
            "/admin/users/get_personal_information/{}",
            target.id
        ))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn admin_get_personal_information_user_not_found() {
    let test_db = TestDatabase::new().await;
    let admin = test_db
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri(&format!(
            "/admin/users/get_personal_information/{}",
            Uuid::new_v4()
        ))
        .insert_header(TestAuthHeader::new(admin))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn admin_users_list_as_default_user_forbidden() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/admin/users/list")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn admin_teams_list_as_default_user_forbidden() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/admin/teams/list")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn admin_email_confirmations_as_default_user_forbidden() {
    let test_db = TestDatabase::new().await;
    let user = test_db.with_default_user().await;

    let app = TestApp::default().with_database(test_db).build_app().await;

    let request = test::TestRequest::get()
        .uri("/admin/users/email_confirmations")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}
