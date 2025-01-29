use actix_web::{get, web, HttpResponse};
use oauth2::reqwest;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use reqwest::redirect::Policy;
use reqwest::{Response, Url};
use serde::Deserialize;

use crate::models::entities::users;
use crate::routes::auth::AuthError::InvalidCredentials;
use crate::routes::auth::TokensResponse;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::error::Error::OAuth;

#[derive(Deserialize, Debug)]
struct QueryParams {
    code: String,
}

#[derive(Deserialize, Debug)]
struct GitHubUser {
    name: String,
    email: Option<String>,
}

#[derive(Deserialize, Debug)]
struct GitHubEmail {
    email: String,
    primary: bool,
}

async fn send_github_request(url: Url, token: &String) -> Result<Response, reqwest::Error> {
    reqwest::Client::new()
        .get(url)
        .header("Authorization", token)
        .header("User-Agent", "hack4krak-backend")
        .send()
        .await
}

#[utoipa::path(
    params(
        ("code" = String, Path)
    ),
    responses(
        (status = 200, description = "OAuth2 flow completed successfully", body = TokensResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server errors."),
    ),
    tag = "auth"
)]
#[get("/oauth/github/callback")]
pub async fn github_callback(
    app_state: web::Data<AppState>,
    data: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .build()?;

    let token_result = app_state
        .github_oauth_client
        .exchange_code(AuthorizationCode::new(data.code.to_string()))
        .request_async(&http_client)
        .await
        .map_err(|_| OAuth)?;

    let token = format!("Bearer {}", token_result.access_token().secret());
    let response_user =
        send_github_request("https://api.github.com/user".parse().unwrap(), &token).await?;

    if !response_user.status().is_success() {
        return Err(Error::Auth(InvalidCredentials));
    }

    let mut user: GitHubUser = response_user.json().await.map_err(|_| OAuth)?;

    if user.email.is_none() {
        let email_response: Vec<GitHubEmail> = send_github_request(
            "https://api.github.com/user/emails".parse().unwrap(),
            &token,
        )
        .await?
        .json()
        .await?;

        if let Some(primary_email) = email_response.iter().find(|email| email.primary) {
            user.email = Some(primary_email.email.clone());
        }
    }

    let Some(email) = user.email else {
        return Err(Error::Auth(InvalidCredentials));
    };

    let tokens = users::Model::create_from_oauth(&app_state.database, user.name, email).await?;

    Ok(HttpResponse::Ok().json(tokens))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Redirects to GitHub for OAuth authorization")
    ),
    tag = "auth"
)]
#[get("/oauth/github")]
pub async fn github(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let (auth_url, _) = app_state
        .github_oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new("user:email".to_string()))
        .url();

    Ok(HttpResponse::Found()
        .insert_header(("Location", auth_url.to_string()))
        .finish())
}
