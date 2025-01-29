use crate::models::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::Claims;
use actix_web::middleware::from_fn;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RemoveUserModel {
    pub username: String,
}

#[utoipa::path(
    request_body = RemoveUserModel,
    responses(
        (status = 200, description = "User successfully removed from team."),
        (status = 403, description = "User is not a member of the team."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/management"
)]
#[post(
    "/manage/remove_user",
    wrap = "from_fn(crate::middlewares::auth_middleware::check_auth_middleware)"
)]
pub async fn remove_user(
    app_state: web::Data<app_state::AppState>,
    remove_user_model: web::Json<RemoveUserModel>,
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    teams::Model::remove_user(&app_state.database, remove_user_model.0, claim_data).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}
