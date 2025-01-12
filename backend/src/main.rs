mod routes;
mod utils;

use crate::utils::app_state::AppState;
use actix_web::{web, App, HttpServer};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::path::Path;
use tracing::info;
use utoipa::openapi::{Info, License};
use utoipa_actix_web::AppExt;
use utoipa_scalar::{Scalar, Servable};

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

    let ip_env = std::env::var("BACKEND_IP").unwrap();
    let mut ip_addr: Vec<&str> = vec![];
    for addr_part in ip_env.split(":") {
        ip_addr.push(addr_part);
    }

    info!("Starting server...");
    let server = HttpServer::new(move || {
        let (app, mut api) = App::new()
            .into_utoipa_app()
            .app_data(web::Data::new(AppState {
                database: db.clone(),
            }))
            .service(routes::index::index)
            .split_for_parts();

        api.info = Info::builder()
            .title("Hack4Krak")
            .license(Some(License::new("GPL")))
            .version(env!("CARGO_PKG_VERSION"))
            .build();

        app.service(Scalar::with_url("/docs", api))
    })
    .bind((ip_addr[0], ip_addr[1].parse::<u16>().unwrap()))?
    .run();

    info!("Server is running on {}", ip_env);
    server.await?;

    info!("Server stopped");
    Ok(())
}
