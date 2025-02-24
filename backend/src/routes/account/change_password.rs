use crate::entities::users;
use crate::middlewares::auth::AuthMiddleware;
use crate::services::auth::AuthService;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, patch};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
struct ChangePasswordModel {
    new_password: String,
    old_password: String,
}

#[utoipa::path(
    request_body = ChangePasswordModel,
    responses(
        (status = 200, description = "Password successfully updated."),
        (status = 401, description = "Old password is incorrect."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "account"
)]
#[patch("/change_password", wrap = "AuthMiddleware::with_user()")]
pub async fn change_password(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    model: Json<ChangePasswordModel>,
) -> Result<HttpResponse, Error> {
    let model = model.into_inner();

    AuthService::change_password(&app_state, user, model.new_password, model.old_password).await?;

    Ok(HttpResponse::Ok().finish())
}
