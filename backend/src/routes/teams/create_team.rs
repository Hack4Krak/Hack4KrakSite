use actix_web::middleware::from_fn;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

use crate::models::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::Claims;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct CreateTeamModel {
    pub team_name: String,
}

#[utoipa::path(
    request_body = CreateTeamModel,
    responses(
        (status = 200, description = "Team successfully created."),
        (status = 403, description = "User already belongs to team."),
        (status = 409, description = "Team already exists."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams"
)]
#[post(
    "/create_team",
    wrap = "from_fn(crate::middlewares::auth_middleware::check_auth_middleware)"
)]
pub async fn create_team(
    app_state: web::Data<app_state::AppState>,
    create_team_model: web::Json<CreateTeamModel>,
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    teams::Model::create_team(&app_state.database, create_team_model.0, claim_data).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}
