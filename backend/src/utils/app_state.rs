use crate::services::env::EnvConfig;
use crate::services::task_manager::TaskManager;
use crate::utils::oauth::OAuthProvider;
use crate::utils::sse_event::SseEvent;
use lettre::SmtpTransport;
use lettre::transport::smtp::authentication::Credentials;
use sea_orm::DatabaseConnection;
use tokio::sync::broadcast;

const LEADERBOARD_UPDATES_CHANNEL_CAPACITY: i8 = 4;

pub struct AppState {
    pub database: DatabaseConnection,
    pub task_manager: TaskManager,
    pub github_oauth_provider: OAuthProvider,
    pub google_oauth_provider: OAuthProvider,
    pub smtp_client: SmtpTransport,
    pub sse_event_sender: broadcast::Sender<SseEvent>,
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

        let smtp_client = SmtpTransport::relay("smtp.resend.com")
            .unwrap()
            .credentials(Credentials::new(
                "resend".to_string(),
                config.resend_api_key.clone(),
            ))
            .build();

        let (leaderboard_updates_transmitter, _) =
            broadcast::channel(LEADERBOARD_UPDATES_CHANNEL_CAPACITY as usize);

        AppState {
            task_manager,
            database,
            github_oauth_provider,
            google_oauth_provider,
            smtp_client,
            sse_event_sender: leaderboard_updates_transmitter,
        }
    }

    pub fn with_database(database: DatabaseConnection) -> AppState {
        AppState {
            database,
            ..Default::default()
        }
    }

    pub fn with_email_client(smtp_client: SmtpTransport) -> AppState {
        AppState {
            smtp_client,
            ..Default::default()
        }
    }

    pub fn with_database_and_smtp_client(
        database: DatabaseConnection,
        smtp_client: SmtpTransport,
    ) -> AppState {
        AppState {
            database,
            smtp_client,
            ..Default::default()
        }
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
            smtp_client: SmtpTransport::relay("email.example.com").unwrap().build(),
            sse_event_sender: broadcast::channel(LEADERBOARD_UPDATES_CHANNEL_CAPACITY as usize).0,
        }
    }
}
