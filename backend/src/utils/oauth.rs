use crate::entities::users;
use crate::services::auth::AuthService;
use crate::services::env::EnvConfig;
use crate::utils::error::Error;
use crate::utils::error::Error::OAuth;
use oauth2::basic::*;
use oauth2::*;
use reqwest::redirect::Policy;
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

#[derive(Clone)]
pub struct OAuthProvider(OAuthClient);

impl OAuthProvider {
    pub fn new(
        client_id: String,
        client_secret: String,
        auth_uri: &str,
        token_uri: &str,
        redirect_url: &str,
    ) -> Self {
        let client_id = ClientId::new(client_id);
        let client_secret = ClientSecret::new(client_secret);
        let redirect_url = RedirectUrl::new(redirect_url.to_string()).unwrap();

        let client = BasicClient::new(client_id)
            .set_client_secret(client_secret)
            .set_auth_uri(AuthUrl::new(auth_uri.to_string()).unwrap())
            .set_token_uri(TokenUrl::new(token_uri.to_string()).unwrap())
            .set_redirect_uri(redirect_url);
        Self(client)
    }

    pub fn redirect_response(&self, scopes: Vec<String>) -> actix_web::HttpResponse {
        let mut client = self.0.authorize_url(CsrfToken::new_random);
        for scope in scopes {
            client = client.add_scope(Scope::new(scope));
        }
        let redirect_url = client.url().0;

        actix_web::HttpResponse::Found()
            .insert_header(("Location", redirect_url.to_string()))
            .finish()
    }

    pub async fn exchange_code(&self, code: String) -> Result<String, Error> {
        let http_client = reqwest::ClientBuilder::new()
            .redirect(Policy::none())
            .build()?;

        let token_result = self
            .0
            .exchange_code(AuthorizationCode::new(code))
            .request_async(&http_client)
            .await
            .map_err(|_| OAuth)?;

        Ok(format!("Bearer {}", token_result.access_token().secret()))
    }

    pub async fn finish_response(
        database: &DatabaseConnection,
        username: String,
        email: String,
    ) -> Result<actix_web::HttpResponse, Error> {
        let user = users::Model::create_from_oauth(database, username, email).await?;

        let mut response = actix_web::HttpResponse::Ok();
        response.append_header((
            "Refresh",
            format!("0; {}", EnvConfig::get().oauth_finish_redirect_url.clone()),
        ));
        AuthService::append_tokens_as_cookies(user.id, user.email, &mut response)?;
        Ok(response.body("Redirecting..."))
    }
}
