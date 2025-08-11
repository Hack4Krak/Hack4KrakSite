pub mod database;
pub mod header;
pub mod mail;

use actix_http::Request;
use actix_web::body::MessageBody;
use actix_web::dev::{Service, ServiceResponse};
use actix_web::web::Data;
use actix_web::{Error, test};
use database::TestDatabase;
use hack4krak_backend::entities::*;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::services::task_manager::TaskManager;
use hack4krak_backend::setup_actix_app;
use hack4krak_backend::utils::app_state::AppState;
use lettre::SmtpTransport;
use migration::TableCreateStatement;
use sea_orm::{
    ConnectionTrait, Database, DatabaseConnection, DbBackend, DbConn, EntityTrait, Schema,
};

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
    setup_schema(&database, user_personal_info::Entity).await;
    setup_schema(&database, email_verification_request::Entity).await;
    setup_schema(&database, external_team_invitation::Entity).await;
    setup_schema(&database, flag_capture::Entity).await;

    // We have to manually create all indexes
    database
        .execute_unprepared("CREATE UNIQUE INDEX unique_team_task ON flag_capture (team, task);")
        .await
        .unwrap();

    database
}

#[derive(Default)]
pub struct TestApp {
    pub database: Option<TestDatabase>,
    pub smtp_client: Option<SmtpTransport>,
    pub task_manager: Option<TaskManager>,
}

impl TestApp {
    pub fn with_database(mut self, test_database: TestDatabase) -> Self {
        self.database = Some(test_database);
        self
    }

    pub fn with_task_manager(mut self, task_manager: TaskManager) -> Self {
        self.task_manager = Some(task_manager);
        self
    }

    pub fn with_smtp_client(mut self, smtp_client: SmtpTransport) -> Self {
        self.smtp_client = Some(smtp_client);
        self
    }

    pub async fn build_app(
        self,
    ) -> impl Service<Request, Response = ServiceResponse<impl MessageBody>, Error = Error> {
        EnvConfig::load_test_config();

        let database = match self.database {
            Some(database) => database,
            None => TestDatabase::new().await,
        };

        let mut app_state = AppState::with_database(database.database);
        if let Some(email_client) = self.smtp_client {
            app_state.smtp_client = email_client;
        }
        if let Some(task_manager) = self.task_manager {
            app_state.task_manager = task_manager;
        }

        let factory = setup_actix_app(false)
            .into_app()
            .app_data(Data::new(app_state));

        test::init_service(factory).await
    }
}
