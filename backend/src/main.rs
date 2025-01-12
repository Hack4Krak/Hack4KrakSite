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

    let address_env: String =
        std::env::var("BACKEND_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());
    let address_vec: Vec<&str> = address_env.split(":").collect();
    let ip = address_vec[0];
    let port = address_vec[1]
        .parse::<u16>()
        .expect("The port in BACKEND_IP must be a valid u16 integer");

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
    .bind((ip, port))?
    .run();

    info!("Server is running on {}", address_vec.join(":"));
    server.await?;

    info!("Server stopped");
    Ok(())
}
