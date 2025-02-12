use actix_web::body::MessageBody;
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::web::Data;
use actix_web::{App, Error};
use hack4krak_backend::entities::{team_invites, teams, users};
use hack4krak_backend::setup_actix_app;
use hack4krak_backend::utils::app_state::AppState;
use migration::TableCreateStatement;
use sea_orm::{ConnectionTrait, Database, DbBackend, DbConn, EntityTrait, Schema};

async fn setup_schema(database: &DbConn, entity: impl EntityTrait) {
    let schema = Schema::new(DbBackend::Sqlite);

    let stmt: TableCreateStatement = schema.create_table_from_entity(entity);
    database
        .execute(database.get_database_backend().build(&stmt))
        .await
        .unwrap();
}

pub async fn setup_test_app() -> App<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    let database = Database::connect("sqlite::memory:").await.unwrap();

    setup_schema(&database, team_invites::Entity).await;
    setup_schema(&database, teams::Entity).await;
    setup_schema(&database, users::Entity).await;

    let state = Data::new(AppState::with_database(database));
    setup_actix_app(false).into_app().app_data(state.clone())
}
