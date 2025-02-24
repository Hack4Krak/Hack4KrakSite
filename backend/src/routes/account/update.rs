use crate::entities::users;
use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, patch};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate, Clone)]
pub struct UpdateUserModel {
    #[validate(length(min = 3, max = 32))]
    pub username: Option<String>,
    #[validate(email)]
    pub email: Option<String>,
}

#[utoipa::path(
    request_body = UpdateUserModel,
    responses(
        (status = 200, description = "Account updated successfully"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("access_token" = [])
    ),
    tag = "account"
)]
#[patch("/account/update")]
pub async fn update(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    Validated(model): Validated<Json<UpdateUserModel>>,
) -> Result<HttpResponse, Error> {
    let model = model.into_inner();

    users::Model::update(&app_state.database, user.clone(), model.clone()).await?;

    if let Some(email) = model.email {
        return AuthService::response_with_cookies(user.id, email);
    }

    Ok(HttpResponse::Ok().finish())
}
