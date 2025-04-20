use crate::entities::users;
use crate::entities::users::UpdatableModel;
use crate::middlewares::auth::AuthMiddleware;
use crate::models::user::Password;
use crate::models::user::validate_username_chars;
use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, patch};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Serialize, Deserialize, ToSchema, Validate, Debug, Default)]
struct UpdateUserModel {
    #[validate(
        length(min = 3, max = 32),
        custom(function = "validate_username_chars")
    )]
    pub username: String,
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
#[patch("/update", wrap = "AuthMiddleware::with_user()")]
pub async fn update(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    Validated(model): Validated<Json<UpdateUserModel>>,
) -> Result<HttpResponse, Error> {
    let model = model.into_inner();

    let updatable_model = UpdatableModel {
        username: Some(model.username),
        ..Default::default()
    };

    users::Model::update(&app_state.database, user, updatable_model).await?;

    Ok(SuccessResponse::default().http_response())
}

#[derive(Serialize, Deserialize, ToSchema, Validate, Clone, Debug, Default)]
struct ChangePasswordModel {
    #[validate(length(min = 8, max = 32))]
    pub old_password: Password,
    #[validate(length(min = 8, max = 32))]
    pub new_password: Password,
}

#[utoipa::path(
    request_body = ChangePasswordModel,
    responses(
        (status = 200, description = "Account updated successfully"),
        (status = 500, description = "Internal server error"),
    ),
    security(
        ("access_token" = [])
    ),
    tag = "account"
)]
#[patch("/update/password", wrap = "AuthMiddleware::with_user()")]
pub async fn change_password(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    Validated(model): Validated<Json<ChangePasswordModel>>,
) -> Result<HttpResponse, Error> {
    let model = model.into_inner();

    AuthService::assert_password_is_valid(&user, &model.old_password)?;

    let hashed_password = AuthService::hash_password(model.new_password.clone())?;

    let updatable_model = UpdatableModel {
        password: Some(Some(hashed_password)),
        ..Default::default()
    };

    users::Model::update(&app_state.database, user, updatable_model).await?;

    Ok(SuccessResponse::default().http_response())
}
