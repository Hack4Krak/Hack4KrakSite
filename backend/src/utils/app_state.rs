use crate::utils::task::TaskManager;
use crossbeam::sync::ShardedLock;
use lettre::SmtpTransport;
use oauth2::basic::*;
use oauth2::{
    AuthUrl, Client, ClientId, EndpointNotSet, EndpointSet, RedirectUrl, StandardRevocableToken,
    TokenUrl,
};
use sea_orm::DatabaseConnection;
use std::collections::HashMap;

type OAuthClient = Client<
    BasicErrorResponse,
    BasicTokenResponse,
    BasicTokenIntrospectionResponse,
    StandardRevocableToken,
    BasicRevocationErrorResponse,
    EndpointSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointNotSet,
    EndpointSet,
>;

pub struct AppState {
    pub database: DatabaseConnection,
    pub task_manager: ShardedLock<TaskManager>,
    pub github_oauth_client: OAuthClient,
    pub google_oauth_client: OAuthClient,
    pub smtp_client: SmtpTransport,
}

impl Default for AppState {
    fn default() -> Self {
        let oauth_client: OAuthClient = BasicClient::new(ClientId::new("test".to_string()))
            .set_auth_uri(AuthUrl::new("https://authorize".to_string()).unwrap())
            .set_token_uri(TokenUrl::new("https://token".to_string()).unwrap())
            .set_redirect_uri(RedirectUrl::new("https://redirect".to_string()).unwrap());

        AppState {
            database: Default::default(),
            task_manager: ShardedLock::new(TaskManager {
                tasks: HashMap::new(),
            }),
            github_oauth_client: oauth_client.clone(),
            google_oauth_client: oauth_client,
            smtp_client: SmtpTransport::relay("email.example.com").unwrap().build(),
        }
    }
}

impl AppState {
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
}
