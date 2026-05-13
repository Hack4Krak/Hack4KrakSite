use crate::services::env::EnvConfig;
use crate::services::task_manager::TaskManager;
use crate::utils::email::SmtpClient;
use crate::utils::oauth::OAuthProvider;
use crate::utils::points_counter::PointsCounter;
use crate::utils::server_event::BroadcastEnvelope;
use lettre::SmtpTransport;
use sea_orm::DatabaseConnection;
use std::sync::Arc;
use tokio::sync::{RwLock, broadcast};

const SERVER_EVENTS_CHANNEL_CAPACITY: usize = 64;

pub struct AppState {
    pub database: DatabaseConnection,
    pub task_manager: TaskManager,
    pub github_oauth_provider: OAuthProvider,
    pub google_oauth_provider: OAuthProvider,
    pub smtp_client: Arc<dyn SmtpClient>,
    pub server_event_sender: broadcast::Sender<BroadcastEnvelope>,
    pub points_cache: RwLock<Option<PointsCounter>>,
}

impl AppState {
    pub async fn setup(database: DatabaseConnection) -> AppState {
        let config = EnvConfig::get();

        let github_oauth_provider = OAuthProvider::new(
            config.github_oauth_client_id.clone(),
            config.github_oauth_client_secret.clone(),
            "https://github.com/login/oauth/authorize",
            "https://github.com/login/oauth/access_token",
            EnvConfig::get()
                .backend_url
                .join("/auth/oauth/github/callback")
                .unwrap()
                .as_str(),
        );

        let google_oauth_provider = OAuthProvider::new(
            config.google_oauth_client_id.clone(),
            config.google_oauth_client_secret.clone(),
            "https://accounts.google.com/o/oauth2/v2/auth",
            "https://www.googleapis.com/oauth2/v3/token",
            EnvConfig::get()
                .backend_url
                .join("/auth/oauth/google/callback")
                .unwrap()
                .as_str(),
        );

        let task_manager = TaskManager::load().await;

        let smtp_client = SmtpTransport::from_url(&config.smtp_connection_url)
            .expect("Invalid SMTP connection URL")
            .build();

        let (server_event_sender, _) = broadcast::channel(SERVER_EVENTS_CHANNEL_CAPACITY);

        AppState {
            task_manager,
            database,
            github_oauth_provider,
            google_oauth_provider,
            smtp_client: Arc::new(smtp_client),
            server_event_sender,
            points_cache: RwLock::new(None),
        }
    }

    pub fn with_database(database: DatabaseConnection) -> AppState {
        AppState {
            database,
            ..Default::default()
        }
    }

    pub fn with_email_client(smtp_client: impl SmtpClient + 'static) -> AppState {
        AppState {
            smtp_client: Arc::new(smtp_client),
            ..Default::default()
        }
    }

    pub fn with_database_and_smtp_client(
        database: DatabaseConnection,
        smtp_client: impl SmtpClient + 'static,
    ) -> AppState {
        AppState {
            database,
            smtp_client: Arc::new(smtp_client),
            ..Default::default()
        }
    }

    pub async fn invalidate_points_cache(&self) {
        *self.points_cache.write().await = None;
    }
}

impl Default for AppState {
    fn default() -> Self {
        let oauth_provider = OAuthProvider::new(
            "test".to_string(),
            "skibidi".to_string(),
            "https://authorize",
            "https://token",
            "https://redirect",
        );

        AppState {
            database: Default::default(),
            task_manager: TaskManager::default(),
            github_oauth_provider: oauth_provider.clone(),
            google_oauth_provider: oauth_provider,
            smtp_client: Arc::new(SmtpTransport::relay("email.example.com").unwrap().build()),
            server_event_sender: broadcast::channel(SERVER_EVENTS_CHANNEL_CAPACITY).0,
            points_cache: RwLock::new(None),
        }
    }
}
