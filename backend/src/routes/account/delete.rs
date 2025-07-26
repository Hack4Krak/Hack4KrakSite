use crate::entities::users;
use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, delete};
use sea_orm::ModelTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "Account successfully deleted"),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "account"
)]
#[delete("/delete")]
pub async fn delete(
    app_state: Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    user.delete(&app_state.database).await?;

    Ok(AuthService::reset_cookies_response())
}
