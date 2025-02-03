use crate::utils::task::TaskManager;
use crossbeam::sync::ShardedLock;
use lettre::transport::smtp::authentication::Credentials;
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

impl AppState {
    pub fn with_database(database: DatabaseConnection) -> AppState {
        let oauth_client: OAuthClient = BasicClient::new(ClientId::new("test".to_string()))
            .set_auth_uri(AuthUrl::new("https://authorize".to_string()).unwrap())
            .set_token_uri(TokenUrl::new("https://token".to_string()).unwrap())
            .set_redirect_uri(RedirectUrl::new("https://redirect".to_string()).unwrap());

        AppState {
            task_manager: ShardedLock::new(TaskManager {
                tasks: HashMap::new(),
            }),
            database,
            github_oauth_client: oauth_client.clone(),
            google_oauth_client: oauth_client,
            smtp_client: SmtpTransport::relay("smtp.resend.com")
                .unwrap()
                .credentials(Credentials::new(
                    "resend".to_string(),
                    "resend-api-key".to_string(),
                ))
                .build(),
        }
    }
}
