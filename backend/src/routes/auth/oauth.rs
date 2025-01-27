use crate::models::entities::users;
use crate::routes::auth::TokensResponse;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::error::Error::InvalidCredentials;
use actix_web::{get, web, HttpResponse};
use oauth2::reqwest;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use reqwest::redirect::Policy;
use serde::Deserialize;

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

#[utoipa::path(
    params(
        ("code" = String, Path)
    ),
    responses(
        (status = 200, description = "OAuth2 flow completed successfully", body = TokensResponse),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server errors."),
    )
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
        .map_err(|_| Error::OAuth)?;

    let token = format!("Bearer {}", token_result.access_token().secret());
    let response_user = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header("Authorization", &token)
        .header("User-Agent", "hack4krak-backend")
        .send()
        .await?;

    if !response_user.status().is_success() {
        return Err(Error::InvalidCredentials);
    }

    let mut user: GitHubUser = response_user.json().await.map_err(|_| InvalidCredentials)?;

    if user.email.is_none() {
        let email_response: Vec<GitHubEmail> = reqwest::Client::new()
            .get("https://api.github.com/user/emails")
            .header("Authorization", &token)
            .header("User-Agent", "hack4krak-backend")
            .send()
            .await?
            .json()
            .await?;

        if let Some(primary_email) = email_response.iter().find(|email| email.primary) {
            user.email = Some(primary_email.email.clone());
        }
    }

    if user.email.is_none() {
        return Err(InvalidCredentials);
    }

    let tokens =
        users::Model::create_from_oauth(&app_state.database, user.name, user.email.unwrap())
            .await?;

    Ok(HttpResponse::Ok().json(tokens))
}

#[utoipa::path(responses(
    (status = 200, description = "Redirects to GitHub for OAuth authorization"),
))]
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
