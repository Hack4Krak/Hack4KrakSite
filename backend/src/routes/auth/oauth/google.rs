use actix_web::{get, web, HttpResponse};
use oauth2::{AuthorizationCode, CsrfToken, Scope, TokenResponse};
use reqwest::redirect::Policy;
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
pub struct GoogleUser {
    name: String,
    email: String,
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
#[get("/oauth/google/callback")]
pub async fn google_callback(
    app_state: web::Data<AppState>,
    data: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    let http_client = reqwest::ClientBuilder::new()
        .redirect(Policy::none())
        .build()?;

    let token_result = app_state
        .google_oauth_client
        .exchange_code(AuthorizationCode::new(data.code.to_string()))
        .request_async(&http_client)
        .await
        .map_err(|_| OAuth)?;

    let token = format!("Bearer {}", token_result.access_token().secret());
    let response = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .header("Authorization", token)
        .send()
        .await?;

    if !response.status().is_success() {
        return Err(InvalidCredentials.into());
    }

    let user: GoogleUser = response.json().await.map_err(|_| InvalidCredentials)?;
    let tokens =
        users::Model::create_from_oauth(&app_state.database, user.name, user.email).await?;

    Ok(HttpResponse::Ok().json(tokens))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Redirects to Google for OAuth authorization"),
    ),
    tag = "auth"
)]
#[get("/oauth/google")]
pub async fn google(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    let (auth_url, _) = app_state
        .google_oauth_client
        .authorize_url(CsrfToken::new_random)
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.email".to_string(),
        ))
        .add_scope(Scope::new(
            "https://www.googleapis.com/auth/userinfo.profile".to_string(),
        ))
        .url();

    Ok(HttpResponse::Found()
        .insert_header(("Location", auth_url.to_string()))
        .finish())
}
