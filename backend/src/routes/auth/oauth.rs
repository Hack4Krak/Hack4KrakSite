use crate::models::entities::users;
use crate::routes::auth::refresh::RefreshToken;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::{get, web, HttpResponse};
use oauth2::reqwest::async_http_client;
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use serde::Deserialize;

#[derive(Deserialize, Debug)]
struct QueryParams {
    code: String,
}

#[derive(Deserialize, Debug)]
struct GitHubUser {
    name: String,
    email: String,
}

#[utoipa::path(
    request_body = RefreshToken,
    responses(
        (status = 200, description = "OAuth2 flow completed successfully"),
        (status = 500, description = "Internal server errors."),
    )
)]
#[get("/oauth/github/callback")]
pub async fn github_callback(
    app_state: web::Data<AppState>,
    data: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    let token_result = app_state
        .github_oauth_client
        .exchange_code(AuthorizationCode::new(data.code.to_string()))
        .request_async(async_http_client)
        .await
        .map_err(|_| Error::OAuth)?;

    let token = format!("Bearer {}", token_result.access_token().secret());
    let response = reqwest::Client::new()
        .get("https://api.github.com/user")
        .header("Authorization", token)
        .header("User-Agent", "hack4krak-backend")
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(Error::InvalidCredentials);
    }

    let user: GitHubUser = response
        .json()
        .await
        .map_err(|_| Error::InvalidCredentials)?;
    let tokens =
        users::Model::create_from_oauth(&app_state.database, user.name, user.email).await?;

    Ok(HttpResponse::Ok().json(tokens))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Address to verify using oauth"),
    )
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
