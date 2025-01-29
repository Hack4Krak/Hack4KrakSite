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
pub struct ChangeNameModel {
    pub new_name: String,
}

#[utoipa::path(
    request_body = ChangeNameModel,
    responses(
        (status = 200, description = "Team name successfully changed."),
        (status = 403, description = "User is not the leader of any team."),
        (status = 404, description = "Team not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/management"
)]
#[post(
    "/manage/change_name",
    wrap = "from_fn(crate::middlewares::auth_middleware::check_auth_middleware)"
)]
pub async fn change_name(
    app_state: web::Data<app_state::AppState>,
    change_name_model: web::Json<ChangeNameModel>,
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    teams::Model::change_name(&app_state.database, change_name_model.0, claim_data).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}
