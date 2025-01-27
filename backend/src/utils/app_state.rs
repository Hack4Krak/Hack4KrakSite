use oauth2::basic::*;
use oauth2::{
    AuthUrl, Client, ClientId, EndpointNotSet, EndpointSet, RedirectUrl, StandardRevocableToken,
    TokenUrl,
};
use sea_orm::DatabaseConnection;

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
    pub github_oauth_client: OAuthClient,
    pub google_oauth_client: OAuthClient,
}

impl AppState {
    pub fn with_database(database: DatabaseConnection) -> AppState {
        let oauth_client: OAuthClient = BasicClient::new(ClientId::new("test".to_string()))
            .set_auth_uri(AuthUrl::new("https://authorize".to_string()).unwrap())
            .set_token_uri(TokenUrl::new("https://token".to_string()).unwrap())
            .set_redirect_uri(RedirectUrl::new("https://redirect".to_string()).unwrap());
        AppState {
            database,
            github_oauth_client: oauth_client.clone(),
            google_oauth_client: oauth_client,
        }
    }
}
