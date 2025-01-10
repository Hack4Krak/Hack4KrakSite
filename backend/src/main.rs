mod routes;
mod utils;

use crate::utils::app_state::AppState;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::path::Path;
use tracing::info;
use utoipa::OpenApi;
use utoipa_redoc::{Redoc, Servable};

#[derive(OpenApi)]
#[openapi(paths(routes::index::index))]
struct ApiDoc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::from_path(Path::new("../.env")).unwrap();

    tracing_subscriber::fmt()
        .with_env_filter("hack4krak_backend=trace")
        .init();

    info!("Connecting to db...");
    let database_url = std::env::var("DATABASE_URL").unwrap();
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    info!("Starting server...");
    let server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(AppState {
                database: db.clone(),
            }))
            .service(Redoc::with_url("/openapi", ApiDoc::openapi()))
            .configure(routes::index::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run();

    info!("Server is running on 127.0.0.1:8080");
    server.await?;

    info!("Server stopped");
    Ok(())
}
