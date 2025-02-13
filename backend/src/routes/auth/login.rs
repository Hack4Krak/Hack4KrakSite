use crate::entities::users;
use crate::routes::auth::AuthError::InvalidCredentials;
use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct LoginModel {
    pub email: String,
    pub password: String,
}

#[utoipa::path(
    request_body = LoginModel,
    responses(
        (status = 200, description = "User successfully logged in."),
        (status = 401, description = "Invalid credentials."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "auth"
)]
#[post("/login")]
pub async fn login(
    app_state: web::Data<app_state::AppState>,
    login_json: web::Json<LoginModel>,
) -> Result<HttpResponse, Error> {
    let user = users::Model::find_by_email(&app_state.database, &login_json.email)
        .await?
        .ok_or(Error::Auth(InvalidCredentials))?;

    AuthService::assert_password_is_valid(&user, &login_json.password)?;

    AuthService::response_with_cookies(user.id, user.email)
}
