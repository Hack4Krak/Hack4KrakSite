use crate::test_utils::database::TestDatabase;
use hack4krak_backend::entities::{teams, users};

#[actix_web::test]
async fn test_cannot_remove_team_leader() {
    let test_database = TestDatabase::new().await;
    let team = test_database.with_team(Default::default()).await;
    let user = test_database
        .with_user(users::UpdatableModel {
            is_leader: Some(true),
            team: Some(Some(team.id)),
            ..Default::default()
        })
        .await;

    let result = teams::Model::remove_user(&test_database.database, user).await;
    assert!(result.is_err());
}
