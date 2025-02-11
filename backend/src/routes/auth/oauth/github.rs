use actix_web::{get, web, HttpResponse};
use oauth2::reqwest;
use reqwest::{Response, Url};
use serde::Deserialize;

use crate::routes::auth::AuthError::InvalidCredentials;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::error::Error::OAuth;
use crate::utils::oauth::OAuthProvider;

#[derive(Deserialize, Debug)]
struct QueryParams {
    code: String,
}

#[derive(Deserialize, Debug)]
struct GitHubUser {
    login: String,
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
        (status = 200, description = "OAuth2 flow completed successfully"),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server errors."),
    ),
    tag = "auth/oauth"
)]
#[get("/oauth/github/callback")]
pub async fn github_callback(
    app_state: web::Data<AppState>,
    data: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    let token = app_state
        .github_oauth_provider
        .exchange_code(data.code.to_string())
        .await?;

    println!("1");
    let response =
        send_github_request("https://api.github.com/user".parse().unwrap(), &token).await?;
    if !response.status().is_success() {
        return Err(Error::Auth(InvalidCredentials));
    }

    // panic!("2 {:?} {:?}", response.status(), response.text().await);
    // let hihi =
    let mut user: GitHubUser = response.json().await.map_err(|_| OAuth)?;

    println!("3");

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

    println!("4");

    let Some(email) = user.email else {
        return Err(Error::Auth(InvalidCredentials));
    };

    println!("5");

    OAuthProvider::finish_response(&app_state.database, user.login, email).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Redirects to GitHub for OAuth authorization")
    ),
    tag = "auth/oauth"
)]
#[get("/oauth/github")]
pub async fn github(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(app_state
        .github_oauth_provider
        .redirect_response(vec!["user:email".to_string()]))
}
