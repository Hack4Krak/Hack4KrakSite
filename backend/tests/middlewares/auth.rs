use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::{App, HttpResponse, test, web};
use chrono::Duration;
use hack4krak_backend::entities::sea_orm_active_enums::{TeamStatus, UserRoles};
use hack4krak_backend::entities::{teams, users};
use hack4krak_backend::middlewares::auth::AuthMiddleware;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::jwt::encode_jwt;
use uuid::Uuid;

#[actix_web::test]
async fn middleware_user_is_not_admin() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/admin/")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn middleware_user_is_admin() {
    let test_database = TestDatabase::new().await;
    let user = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Admin),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/admin/")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn no_cookie_returns_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get().uri("/admin/").to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn invalid_jwt_cookie_returns_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/admin/")
        .insert_header(("Cookie", "access_token=invalid.jwt.token"))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn team_member_endpoint_without_team_forbidden() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn team_member_endpoint_with_team_success() {
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

    let request = test::TestRequest::get()
        .uri("/teams/membership/my_team")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn team_leader_endpoint_as_member_forbidden() {
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

    let request = test::TestRequest::patch()
        .uri("/teams/management/rename")
        .insert_header(TestAuthHeader::new(user))
        .set_json(serde_json::json!({ "new_name": "NewTeam" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn team_leader_endpoint_as_leader_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/teams/management/rename")
        .insert_header(TestAuthHeader::new(user))
        .set_json(serde_json::json!({ "new_name": "NewTeam" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn confirmed_team_endpoint_with_unconfirmed_team() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;
    let team = test_database
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Absent),
            ..Default::default()
        })
        .await;
    let user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_confirmed_team_as_member())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn confirmed_team_endpoint_with_confirmed_team() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;
    let team = test_database
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;
    let user = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_confirmed_team_as_member())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn owner_can_access_admin_endpoints() {
    let test_database = TestDatabase::new().await;
    let owner = test_database
        .with_user(users::UpdatableModel {
            roles: Some(UserRoles::Owner),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/admin/")
        .insert_header(TestAuthHeader::new(owner))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn user_deleted_returns_unauthorized() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;
    let jwt = encode_jwt(user.id, user.email.clone(), Duration::minutes(10)).unwrap();

    // Delete the user from the database after creating the JWT
    use sea_orm::EntityTrait;
    users::Entity::delete_by_id(user.id)
        .exec(&test_database.database)
        .await
        .unwrap();

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_user())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(("Cookie", format!("access_token={jwt}")))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn malformed_jwt_cookie_returns_unauthorized() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_user())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(("Cookie", "access_token=not.a.valid.jwt.token"))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn empty_cookie_value_returns_unauthorized() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_user())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(("Cookie", "access_token="))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn expired_jwt_returns_unauthorized() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;
    let expired_jwt = encode_jwt(user.id, user.email.clone(), Duration::minutes(-10)).unwrap();

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_user())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(("Cookie", format!("access_token={expired_jwt}")))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn nonexistent_user_id_in_jwt_returns_unauthorized() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;
    let jwt = encode_jwt(
        Uuid::new_v4(),
        "fake@email.com".to_string(),
        Duration::minutes(10),
    )
    .unwrap();

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_user())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(("Cookie", format!("access_token={jwt}")))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn confirmed_team_leader_can_access() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;
    let team = test_database
        .with_team(teams::UpdatableModel {
            status: Some(TeamStatus::Confirmed),
            ..Default::default()
        })
        .await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_confirmed_team_as_member())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn team_member_endpoint_as_non_member_forbidden() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;
    let _team = test_database.with_default_team().await;
    let user = test_database.with_default_user().await;

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_team_as_member())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn team_leader_endpoint_as_non_leader_forbidden() {
    EnvConfig::load_test_config();
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let member = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(false),
            ..Default::default()
        })
        .await;

    let app_state = AppState::with_database(test_database.database);
    let app = test::init_service(
        App::new().app_data(web::Data::new(app_state)).service(
            web::scope("/test")
                .wrap(AuthMiddleware::with_team_as_leader())
                .route("", web::get().to(|| async { HttpResponse::Ok().finish() })),
        ),
    )
    .await;

    let request = test::TestRequest::get()
        .uri("/test")
        .insert_header(TestAuthHeader::new(member))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}
