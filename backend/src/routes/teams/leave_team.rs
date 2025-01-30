use crate::models::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::Claims;
use actix_web::middleware::from_fn;
use actix_web::{post, web, HttpResponse};
use serde_json::json;

#[utoipa::path(
    responses(
        (status = 200, description = "User successfully left team."),
        (status = 403, description = "User is not a member of the team."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams"
)]
#[post(
    "/leave_team",
    wrap = "from_fn(crate::middlewares::auth_middleware::check_auth_middleware)"
)]
pub async fn remove_user(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    teams::Model::leave_team(&app_state.database, claim_data).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}
