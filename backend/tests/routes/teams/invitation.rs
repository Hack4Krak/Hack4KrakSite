use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use chrono::Local;
use hack4krak_backend::entities::sea_orm_active_enums::{TeamStatus, UserRoles};
use hack4krak_backend::entities::{team_invites, teams, users};
use sea_orm::{ActiveModelTrait, DatabaseConnection, Set};
use uuid::Uuid;

#[actix_web::test]
async fn assert_correct_team_size() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;
    init_database_with_teams(&test_database.database, user.id).await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/teams/invitations/accept_invitation/dziengiel")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 200);
}

#[actix_web::test]
async fn assert_incorrect_team_size() {
    let test_database = TestDatabase::new().await;
    let user = test_database.with_default_user().await;
    let (team_uuid, _) = init_database_with_teams(&test_database.database, user.id).await;
    test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team_uuid)),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/teams/invitations/accept_invitation/dziengiel")
        .insert_header(TestAuthHeader::new(user.clone()))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 409);
}

async fn init_database_with_teams(
    database: &DatabaseConnection,
    user_uuid: Uuid,
) -> (Uuid, Vec<Uuid>) {
    let team_uuid = Uuid::new_v4();

    teams::ActiveModel {
        id: Set(team_uuid),
        name: Set("dziengiel".to_string()),
        created_at: Set(Local::now().naive_local()),
        confirmation_code: Set(Some(team_uuid)),
        status: Set(TeamStatus::Absent),
    }
    .insert(database)
    .await
    .unwrap();

    let users = vec![
        ("Salieri", "example@gmail.com"),
        ("Salieri2", "example2@gmail.com"),
        ("Salieri3", "example3@gmail.com"),
        ("Salieri4", "example4@gmail.com"),
    ];

    let mut users_with_team = Vec::new();

    for (username, email) in users {
        let uuid = Uuid::new_v4();
        users_with_team.push(uuid);

        users::ActiveModel {
            id: Set(uuid),
            username: Set(username.to_string()),
            email: Set(email.to_string()),
            created_at: Set(Local::now().naive_local()),
            is_leader: Set(false),
            roles: Set(UserRoles::Default),
            team: Set(Some(team_uuid)),
            ..Default::default()
        }
        .insert(database)
        .await
        .unwrap();
    }

    team_invites::ActiveModel {
        id: Set(Uuid::new_v4()),
        user: Set(user_uuid),
        team: Set(team_uuid),
    }
    .insert(database)
    .await
    .unwrap();

    (team_uuid, users_with_team)
}
