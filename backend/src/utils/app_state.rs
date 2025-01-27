use oauth2::basic::*;
use oauth2::{
    AuthUrl, Client, ClientId, EndpointNotSet, EndpointSet, RedirectUrl, StandardRevocableToken,
    TokenUrl,
};
use sea_orm::DatabaseConnection;

pub struct AppState {
    pub database: DatabaseConnection,
    pub github_oauth_client: Client<
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
    >,
    pub google_oauth_client: Client<
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
    >,
}

impl AppState {
    pub fn with_database(database: DatabaseConnection) -> AppState {
        AppState {
            database,
            github_oauth_client: BasicClient::new(ClientId::new("test".to_string()))
                .set_auth_uri(AuthUrl::new("https://authorize".to_string()).unwrap())
                .set_token_uri(TokenUrl::new("https://token".to_string()).unwrap())
                .set_redirect_uri(RedirectUrl::new("https://redirect".to_string()).unwrap()),
            google_oauth_client: BasicClient::new(ClientId::new("test".to_string()))
                .set_auth_uri(AuthUrl::new("https://authorize".to_string()).unwrap())
                .set_token_uri(TokenUrl::new("https://token".to_string()).unwrap())
                .set_redirect_uri(RedirectUrl::new("https://redirect".to_string()).unwrap()),
        }
    }
}
