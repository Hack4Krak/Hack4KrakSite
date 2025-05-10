use crate::entities::users;
use crate::middlewares::auth::AuthMiddleware;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::JwtClaims;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserInformationResponse {
    pub username: String,
    pub email: String,
    pub has_personal_information: bool,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Use information received.", body = UserInformationResponse),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "account_index",
    tag = "account"
)]
#[get("/", wrap = "AuthMiddleware::default()")]
pub async fn index(
    app_state: Data<app_state::AppState>,
    claim_data: JwtClaims,
) -> Result<HttpResponse, Error> {
    let user_model = users::Entity::find_by_id(claim_data.id)
        .one(&app_state.database)
        .await?
        .ok_or(Error::Unauthorized)?;

    Ok(HttpResponse::Ok().json(UserInformationResponse {
        email: user_model.email,
        username: user_model.username,
        has_personal_information: user_model.personal_info.is_some(),
    }))
}

#[utoipa::path(
    responses(
        (status = 200, description = "Use information received."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin"
)]
#[get("/admin", wrap = "AuthMiddleware::with_user_as_admin()")]
pub async fn only_admins() -> Result<HttpResponse, Error> {
    Ok(SuccessResponse::default().http_response())
}
