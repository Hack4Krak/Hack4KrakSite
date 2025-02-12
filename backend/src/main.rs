use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{ServiceFactory, ServiceRequest, ServiceResponse};
use actix_web::middleware::Logger;
use actix_web::web::Data;
use actix_web::{App, Error, HttpServer};
use hack4krak_backend::middlewares::status_code_drain_middleware::StatusCodeDrain;
use hack4krak_backend::routes;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::services::task_manager::TaskManager;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::openapi::{write_openapi, ApiDoc};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::time::Instant;
use tracing::info;
use utoipa::OpenApi;
use utoipa_actix_web::{scope, AppExt, UtoipaApp};
use utoipa_scalar::{Scalar, Servable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let start = Instant::now();
    info!("Welcome to the Hack4Krak backend!");

    info!("Initializing env variables...");
    EnvConfig::load_config();
    let (ip, port) = EnvConfig::get().get_ip_and_port();

    info!("Initializing logger...");
    setup_logs();

    info!("Connecting to the database...");
    let database = setup_database().await;

    info!("Loading tasks...");
    let task_manager = TaskManager::load().await;
    info!("Loaded {} tasks from file system", task_manager.tasks.len());

    let app_data = Data::new(AppState::setup(database).await);

    info!("Starting server...");
    let server = HttpServer::new(move || {
        let (app, api) = setup_actix_app()
            .app_data(app_data.clone())
            .split_for_parts();

        write_openapi(&api).expect("Could not generate OpenApi specification file");

        app
    })
    .bind((ip, port))?
    .run();

    let duration = start.elapsed();
    info!(
        "Server started in {:?}. You can connect on {ip}:{port}",
        duration
    );
    server.await?;

    info!("Server stopped. Goodbye :c");
    Ok(())
}

fn setup_actix_app() -> UtoipaApp<
    impl ServiceFactory<
        ServiceRequest,
        Config = (),
        Response = ServiceResponse<impl MessageBody>,
        Error = Error,
        InitError = (),
    >,
> {
    let governor_middleware = GovernorConfigBuilder::default()
        .seconds_per_request(3)
        .burst_size(5)
        .finish()
        .unwrap();

    let cors_middleware = Cors::default()
        .allowed_origin("http://localhost:3000")
        .allowed_origin("https://hack4krak.pl")
        .allow_any_method()
        .allow_any_header()
        .supports_credentials()
        .max_age(3600);

    App::new()
        .wrap(StatusCodeDrain)
        .wrap(Logger::default())
        .wrap(cors_middleware)
        .into_utoipa_app()
        .openapi(ApiDoc::openapi())
        .service(routes::index::index)
        .service(
            scope("/auth")
                .wrap(Governor::new(&governor_middleware))
                .configure(routes::auth::config),
        )
        .service(scope("/teams").configure(routes::teams::config))
        .service(scope("/tasks").configure(routes::task::config))
        .service(scope("/user").configure(routes::user::config))
        .openapi_service(|api| Scalar::with_url("/docs", api))
}

fn setup_logs() {
    let filter =
        env::var("RUST_LOG").unwrap_or("actix_web=debug,hack4krak_backend=trace".to_string());

    tracing_subscriber::fmt().with_env_filter(filter).init();
}

async fn setup_database() -> DatabaseConnection {
    let database = Database::connect(&EnvConfig::get().database_url)
        .await
        .unwrap();
    Migrator::up(&database, None).await.unwrap();

    database
}
