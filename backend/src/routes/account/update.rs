use crate::entities::users;
use crate::middlewares::auth::AuthMiddleware;
use crate::models::user::Password;
use crate::models::user::macros::*;
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
use crate::create_partial_user_struct;

create_partial_user_struct!(UpdateUserModel { Username, ModelPassword }, old_password);

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

    AuthService::assert_password_is_valid(&user, &model.old_password)?;

    users::Model::update(
        &app_state.database,
        user,
        model.Username,
        model.ModelPassword,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
