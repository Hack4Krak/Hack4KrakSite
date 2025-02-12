use actix_web::web::Data;
use actix_web::HttpServer;
use hack4krak_backend::services::env::EnvConfig;
use hack4krak_backend::services::task_manager::TaskManager;
use hack4krak_backend::setup_actix_app;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::openapi::write_openapi;
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::time::Instant;
use tracing::info;

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
