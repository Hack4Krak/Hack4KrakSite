use crate::test_utils::TestApp;
use crate::test_utils::database::TestDatabase;
use crate::test_utils::header::TestAuthHeader;
use actix_web::test;
use hack4krak_backend::entities::users;
use serde_json::json;

#[actix_web::test]
async fn rename_team_success() {
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
        .set_json(json!({ "new_name": "NewName" }))
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn rename_team_name_too_short() {
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
        .set_json(json!({ "new_name": "ab" }))
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn rename_team_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/teams/management/rename")
        .set_json(json!({ "new_name": "NewName" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn kick_user_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let member = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            username: Some("member".to_string()),
            email: Some("member@example.com".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/teams/management/kick_user")
        .set_json(json!({ "username": member.username }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn kick_user_cannot_kick_yourself() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
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

    let request = test::TestRequest::delete()
        .uri("/teams/management/kick_user")
        .set_json(json!({ "username": leader.username }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn kick_user_not_found() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
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

    let request = test::TestRequest::delete()
        .uri("/teams/management/kick_user")
        .set_json(json!({ "username": "nonexistent" }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn change_leader_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let member = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            username: Some("member".to_string()),
            email: Some("member@example.com".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/teams/management/change_leader")
        .set_json(json!({ "new_leader_username": member.username }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn change_leader_user_not_in_team() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    test_database
        .with_user(users::UpdatableModel {
            username: Some("outsider".to_string()),
            email: Some("outsider@example.com".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::patch()
        .uri("/teams/management/change_leader")
        .set_json(json!({ "new_leader_username": "outsider" }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn delete_team_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
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

    let request = test::TestRequest::delete()
        .uri("/teams/management/delete")
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn invite_user_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    test_database
        .with_user(users::UpdatableModel {
            username: Some("invitee".to_string()),
            email: Some("invitee@example.com".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/teams/management/invite_user")
        .set_json(json!({ "username": "invitee" }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn invite_user_already_in_team() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            username: Some("member".to_string()),
            email: Some("member@example.com".to_string()),
            ..Default::default()
        })
        .await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/teams/management/invite_user")
        .set_json(json!({ "username": "member" }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 403);
}

#[actix_web::test]
async fn invite_user_not_found() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
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

    let request = test::TestRequest::post()
        .uri("/teams/management/invite_user")
        .set_json(json!({ "username": "ghost" }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn invited_users_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let invitee = test_database
        .with_user(users::UpdatableModel {
            username: Some("invitee".to_string()),
            email: Some("invitee@example.com".to_string()),
            ..Default::default()
        })
        .await;
    test_database.with_invite(invitee.id, team.id).await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/management/invited_users")
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn revoke_invitation_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let invitee = test_database
        .with_user(users::UpdatableModel {
            username: Some("invitee".to_string()),
            email: Some("invitee@example.com".to_string()),
            ..Default::default()
        })
        .await;
    test_database.with_invite(invitee.id, team.id).await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/teams/management/revoke_invitation/invitee")
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn revoke_invitation_not_found() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
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

    let request = test::TestRequest::delete()
        .uri("/teams/management/revoke_invitation/nobody")
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 404);
}

#[actix_web::test]
async fn management_index_success() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
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

    let request = test::TestRequest::get()
        .uri("/teams/management/")
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_success());
}

#[actix_web::test]
async fn rename_team_name_too_long() {
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
        .set_json(json!({ "new_name": "a".repeat(33) }))
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn rename_team_special_chars() {
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
        .set_json(json!({ "new_name": "team@name!" }))
        .insert_header(TestAuthHeader::new(user))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 400);
}

#[actix_web::test]
async fn delete_team_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/teams/management/delete")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn invite_user_already_invited() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
        .with_user(users::UpdatableModel {
            team: Some(Some(team.id)),
            is_leader: Some(true),
            ..Default::default()
        })
        .await;
    let invitee = test_database
        .with_user(users::UpdatableModel {
            username: Some("invitee".to_string()),
            email: Some("invitee@example.com".to_string()),
            ..Default::default()
        })
        .await;
    test_database.with_invite(invitee.id, team.id).await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/teams/management/invite_user")
        .set_json(json!({ "username": "invitee" }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 409);
}

#[actix_web::test]
async fn invite_user_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::post()
        .uri("/teams/management/invite_user")
        .set_json(json!({ "username": "someone" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn change_leader_to_self() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
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
        .uri("/teams/management/change_leader")
        .set_json(json!({ "new_leader_username": leader.username }))
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert!(response.status().is_client_error());
}

#[actix_web::test]
async fn kick_user_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/teams/management/kick_user")
        .set_json(json!({ "username": "someone" }))
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn invited_users_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/management/invited_users")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn invited_users_empty() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_default_team().await;
    let leader = test_database
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

    let request = test::TestRequest::get()
        .uri("/teams/management/invited_users")
        .insert_header(TestAuthHeader::new(leader))
        .to_request();
    let response: Vec<String> = test::call_and_read_body_json(&app, request).await;
    assert!(response.is_empty());
}

#[actix_web::test]
async fn revoke_invitation_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::delete()
        .uri("/teams/management/revoke_invitation/someone")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}

#[actix_web::test]
async fn management_index_unauthorized() {
    let test_database = TestDatabase::new().await;

    let app = TestApp::default()
        .with_database(test_database)
        .build_app()
        .await;

    let request = test::TestRequest::get()
        .uri("/teams/management/")
        .to_request();
    let response = test::call_service(&app, request).await;
    assert_eq!(response.status(), 401);
}
