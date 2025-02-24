use actix_web::{HttpResponse, get, web};
use serde::Deserialize;

use crate::routes::auth::AuthError::InvalidCredentials;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::oauth::OAuthProvider;

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
        (status = 200, description = "OAuth2 flow completed successfully"),
        (status = 401, description = "Invalid credentials"),
        (status = 500, description = "Internal server errors."),
    ),
    tag = "auth/oauth"
)]
#[get("/oauth/google/callback")]
pub async fn google_callback(
    app_state: web::Data<AppState>,
    data: web::Query<QueryParams>,
) -> Result<HttpResponse, Error> {
    let token = app_state
        .google_oauth_provider
        .exchange_code(data.code.to_string())
        .await?;

    let response = reqwest::Client::new()
        .get("https://www.googleapis.com/oauth2/v3/userinfo")
        .header("Authorization", token)
        .send()
        .await?;
    if !response.status().is_success() {
        return Err(InvalidCredentials.into());
    }

    let user: GoogleUser = response.json().await.map_err(|_| InvalidCredentials)?;

    OAuthProvider::finish_response(&app_state.database, user.name, user.email).await
}

#[utoipa::path(
    responses(
        (status = 200, description = "Redirects to Google for OAuth authorization"),
    ),
    tag = "auth/oauth"
)]
#[get("/oauth/google")]
pub async fn google(app_state: web::Data<AppState>) -> Result<HttpResponse, Error> {
    Ok(app_state.google_oauth_provider.redirect_response(vec![
        "https://www.googleapis.com/auth/userinfo.email".to_string(),
        "https://www.googleapis.com/auth/userinfo.profile".to_string(),
    ]))
}
