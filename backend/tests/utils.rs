use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::{App, Error};
use hack4krak_backend::entities::{email_confirmation, team_invites, teams, users};
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

pub async fn setup_test_app(
    email_client: Option<SmtpTransport>,
    database_connection: Option<DatabaseConnection>,
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
        None => {
            let database = Database::connect("sqlite::memory:").await.unwrap();

            setup_schema(&database, team_invites::Entity).await;
            setup_schema(&database, teams::Entity).await;
            setup_schema(&database, users::Entity).await;
            setup_schema(&database, email_confirmation::Entity).await;

            database
        }
    };

    if let Some(email_client) = email_client {
        let state = Data::new(AppState::with_database_and_smtp_client(
            database,
            email_client,
        ));
        return setup_actix_app(false).into_app().app_data(state.clone());
    }
    let state = Data::new(AppState::with_database(database));
    setup_actix_app(false).into_app().app_data(state.clone())
}
