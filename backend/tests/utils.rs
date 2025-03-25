use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::{App, Error};
use chrono::Local;
use hack4krak_backend::entities::sea_orm_active_enums::{TeamStatus, UserRoles};
use hack4krak_backend::entities::{
    email_confirmation, flag_capture, password_reset, team_invites, teams, users,
};
use hack4krak_backend::services::task_manager::TaskManager;
use hack4krak_backend::setup_actix_app;
use hack4krak_backend::utils::app_state::AppState;
use lettre::SmtpTransport;
use migration::TableCreateStatement;
use sea_orm::ActiveValue::Set;
use sea_orm::{
    ActiveModelTrait, ConnectionTrait, Database, DatabaseConnection, DbBackend, DbConn,
    EntityTrait, Schema,
};
use uuid::Uuid;

pub async fn setup_schema(database: &DbConn, entity: impl EntityTrait) {
    let schema = Schema::new(DbBackend::Sqlite);

    let stmt: TableCreateStatement = schema.create_table_from_entity(entity);
    database
        .execute(database.get_database_backend().build(&stmt))
        .await
        .unwrap();
}

pub async fn setup_database_with_schema() -> DatabaseConnection {
    let database = Database::connect("sqlite::memory:").await.unwrap();

    setup_schema(&database, team_invites::Entity).await;
    setup_schema(&database, teams::Entity).await;
    setup_schema(&database, users::Entity).await;
    setup_schema(&database, email_confirmation::Entity).await;
    setup_schema(&database, flag_capture::Entity).await;
    setup_schema(&database, password_reset::Entity).await;

    // We have to manually create all indexes
    database
        .execute_unprepared("CREATE UNIQUE INDEX unique_team_task ON flag_capture (team, task);")
        .await
        .unwrap();

    database
}

pub async fn setup_test_app(
    email_client: Option<SmtpTransport>,
    database_connection: Option<DatabaseConnection>,
    task_manager: Option<TaskManager>,
) -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    let database = match database_connection {
        Some(database) => database,
        None => setup_database_with_schema().await,
    };

    let mut app_state = AppState::with_database(database);
    if let Some(email_client) = email_client {
        app_state.smtp_client = email_client;
    }
    if let Some(task_manager) = task_manager {
        app_state.task_manager = task_manager;
    }

    setup_actix_app(false)
        .into_app()
        .app_data(Data::new(app_state))
}

// This will be fixed properly in #189
#[allow(dead_code)]
pub async fn init_database_with_teams() -> (DatabaseConnection, Uuid, Uuid, Vec<Uuid>) {
    let database = setup_database_with_schema().await;

    let team_uuid = Uuid::new_v4();

    teams::ActiveModel {
        id: Set(team_uuid),
        name: Set("dziengiel".to_string()),
        created_at: Set(Local::now().naive_local()),
        confirmation_code: Set(Some(team_uuid)),
        status: Set(TeamStatus::Absent),
    }
    .insert(&database)
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
        .insert(&database)
        .await
        .unwrap();
    }

    let user_uuid = Uuid::new_v4();

    users::ActiveModel {
        id: Set(user_uuid),
        username: Set("Antonio".to_string()),
        email: Set("skibidi@gmail.com".to_string()),
        created_at: Set(Local::now().naive_local()),
        is_leader: Set(false),
        roles: Set(UserRoles::Default),
        ..Default::default()
    }
    .insert(&database)
    .await
    .unwrap();

    team_invites::ActiveModel {
        id: Set(Uuid::new_v4()),
        user: Set(user_uuid),
        team: Set(team_uuid),
    }
    .insert(&database)
    .await
    .unwrap();

    (database, user_uuid, team_uuid, users_with_team)
}

#[allow(dead_code)]
pub async fn init_database_with_user() -> (DatabaseConnection, Uuid) {
    let database = setup_database_with_schema().await;

    let uuid = Uuid::new_v4();

    users::Entity::insert(users::ActiveModel {
        id: Set(uuid),
        username: Set("test_user".to_string()),
        email: Set("example@gmail.com".to_string()),
        created_at: Set(Local::now().naive_local()),
        team: Set(None),
        is_leader: Set(false),
        // Password is Dziengiel
        password: Set(Some("$argon2id$v=19$m=19456,t=2,p=1$GuyDKoLJCF5tt+MDGJqRfA$8NZPkyNbR/IWuLg6tR7tn0RH/lJGahLYDODj23ajP3Y".to_string())),
        roles: Set(UserRoles::Default),
    })
    .exec(&database)
    .await
    .unwrap();

    (database, uuid)
}
