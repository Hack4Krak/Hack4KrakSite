use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use actix_web::web::Data;
use actix_web::{middleware::from_fn, middleware::Logger};
use actix_web::{App, HttpServer};
use crossbeam::sync::ShardedLock;
use hack4krak_backend::utils::app_state::AppState;
use hack4krak_backend::utils::env::Config;
use hack4krak_backend::utils::openapi::ApiDoc;
use hack4krak_backend::utils::task::TaskManager;
use hack4krak_backend::{middlewares, routes};
use lettre::transport::smtp::authentication::Credentials;
use lettre::SmtpTransport;
use migration::{Migrator, MigratorTrait};
use oauth2::basic::BasicClient;
use oauth2::{AuthUrl, ClientId, ClientSecret, RedirectUrl, TokenUrl};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use std::fs::File;
use std::io::Write;
use tracing::info;
use utoipa::gen::serde_json::to_string;
use utoipa::OpenApi;
use utoipa_actix_web::{scope, AppExt};
use utoipa_scalar::{Scalar, Servable};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    Config::load_config();

    let filter =
        env::var("RUST_LOG").unwrap_or("actix_web=debug,hack4krak_backend=trace".to_string());

    tracing_subscriber::fmt().with_env_filter(filter).init();

    let governor_configuration = GovernorConfigBuilder::default()
        .seconds_per_request(3)
        .burst_size(5)
        .finish()
        .unwrap();

    info!("Connecting to db...");
    let db: DatabaseConnection = Database::connect(&Config::get().database_url)
        .await
        .unwrap();
    Migrator::up(&db, None).await.unwrap();

    let address_env = &Config::get().backend_address;
    let address_vec: Vec<&str> = address_env.split(":").collect();
    let ip = address_vec[0];
    let port = address_vec[1]
        .parse::<u16>()
        .expect("The port in BACKEND_ADDRESS must be a valid u16 integer");

    let github_client_id = ClientId::new(Config::get().github_oauth_client_id.clone());
    let github_client_secret = ClientSecret::new(Config::get().github_oauth_client_secret.clone());
    let github_redirect_url =
        RedirectUrl::new(Config::get().github_oauth_redirect_url.clone()).unwrap();
    let github_oauth_client = BasicClient::new(github_client_id)
        .set_client_secret(github_client_secret)
        .set_auth_uri(AuthUrl::new("https://github.com/login/oauth/authorize".to_string()).unwrap())
        .set_token_uri(
            TokenUrl::new("https://github.com/login/oauth/access_token".to_string()).unwrap(),
        )
        .set_redirect_uri(github_redirect_url);

    let google_oauth_client_id = ClientId::new(Config::get().google_oauth_client_id.clone());
    let google_oauth_client_secret =
        ClientSecret::new(Config::get().google_oauth_client_secret.clone());
    let google_redirect_url =
        RedirectUrl::new(Config::get().google_oauth_redirect_url.clone()).unwrap();
    let google_oauth_client = BasicClient::new(google_oauth_client_id)
        .set_client_secret(google_oauth_client_secret)
        .set_auth_uri(
            AuthUrl::new("https://accounts.google.com/o/oauth2/v2/auth".to_string()).unwrap(),
        )
        .set_token_uri(
            TokenUrl::new("https://www.googleapis.com/oauth2/v3/token".to_string()).unwrap(),
        )
        .set_redirect_uri(google_redirect_url);

    let task_manager = ShardedLock::new(TaskManager::load().await);

    let smtp_client = SmtpTransport::relay("smtp.resend.com")
        .unwrap()
        .credentials(Credentials::new(
            "resend".to_string(),
            Config::get().resend_api_key.clone(),
        ))
        .build();

    let data = Data::new(AppState {
        task_manager,
        database: db,
        github_oauth_client,
        google_oauth_client,
        smtp_client,
    });

    info!("Starting server...");
    let server = HttpServer::new(move || {
        let (app, api) = App::new()
            .wrap(Logger::default())
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:3000")
                    .allowed_origin("https://hack4krak.pl")
                    .allow_any_method()
                    .allow_any_header()
                    .supports_credentials()
                    .max_age(3600),
            )
            .into_utoipa_app()
            .openapi(ApiDoc::openapi())
            .app_data(data.clone())
            .service(routes::index::index)
            .service(
                scope("/auth")
                    .wrap(Governor::new(&governor_configuration))
                    .configure(routes::auth::config),
            )
            .service(scope("/teams").configure(routes::teams::config))
            .service(scope("/tasks").configure(routes::task::config))
            .service(
                scope("/teams/invitations")
                    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .configure(routes::teams::invitations::config),
            )
            .service(
                scope("/teams/manage")
                    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .configure(routes::teams::management::config),
            )
            .service(
                scope("/user")
                    .wrap(from_fn(middlewares::auth_middleware::check_auth_middleware))
                    .configure(routes::user::config),
            )
            .openapi_service(|api| Scalar::with_url("/docs", api))
            .split_for_parts();

        if cfg!(debug_assertions) {
            let path = &Config::get().openapi_json_frontend_path;
            let mut openapi_json = File::create(path).unwrap();
            openapi_json
                .write_all(to_string(&api).unwrap().as_bytes())
                .unwrap();
        }

        app
    })
    .bind((ip, port))?
    .run();

    info!("Server is running on {}", address_vec.join(":"));
    server.await?;

    info!("Server stopped");
    Ok(())
}
