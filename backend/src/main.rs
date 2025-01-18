use actix_web::web::Data;
use actix_web::{middleware::from_fn, middleware::Logger};
use actix_web::{App, HttpServer};
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::{middlewares, routes};
use migration::{Migrator, MigratorTrait};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::Path;
use tracing::info;
use utoipa::gen::serde_json::to_string;
use utoipa::openapi::security::{HttpAuthScheme, HttpBuilder, SecurityScheme};
use utoipa::openapi::{Info, License};
use utoipa_actix_web::{scope, AppExt};
use utoipa_scalar::{Scalar, Servable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::from_path(Path::new("../.env")).unwrap();

    let filter = env::var("RUST_LOG").unwrap_or("actix_web=debug,hack4krak_backend=trace".to_string());

    tracing_subscriber::fmt().with_env_filter(filter).init();

    info!("Connecting to db...");
    let database_url = env::var("DATABASE_URL").unwrap();
    let db: DatabaseConnection = Database::connect(database_url).await.unwrap();
    Migrator::up(&db, None).await.unwrap();

    let address_env: String = env::var("BACKEND_ADDRESS").unwrap_or("127.0.0.1:8080".to_string());
    let address_vec: Vec<&str> = address_env.split(":").collect();
    let ip = address_vec[0];
    let port = address_vec[1]
        .parse::<u16>()
        .expect("The port in BACKEND_ADDRESS must be a valid u16 integer");

    let data = Data::new(AppState { database: db });

    info!("Starting server...");
    let server = HttpServer::new(move || {
        let (app, mut api) = App::new()
            .wrap(Logger::default())
            .into_utoipa_app()
            .app_data(data.clone())
            .service(routes::index::index)
            .service(scope("/auth").configure(routes::auth::config))
            .service(
                scope("/user")
                    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .configure(routes::user::config),
            )
            .split_for_parts();

        api.info = Info::builder()
            .title("Hack4Krak")
            .license(Some(License::new("GPL")))
            .version(env!("CARGO_PKG_VERSION"))
            .build();

        if let Some(ref mut components) = api.components {
            components.add_security_scheme(
                "access_token",
                SecurityScheme::Http(
                    HttpBuilder::new()
                        .scheme(HttpAuthScheme::Bearer)
                        .bearer_format("JWT")
                        .build(),
                ),
            );
        }

        let path: String = env::var("OPENAPI_JSON_FRONTEND_PATH")
            .unwrap_or("../frontend/openapi/api/openapi.json".to_string());
        let mut openapi_json = File::create(path).unwrap();
        openapi_json
            .write_all(to_string(&api).unwrap().as_bytes())
            .unwrap();
        app.service(Scalar::with_url("/docs", api))
    })
    .bind((ip, port))?
    .run();

    info!("Server is running on {}", address_vec.join(":"));
    server.await?;

    info!("Server stopped");
    Ok(())
}
