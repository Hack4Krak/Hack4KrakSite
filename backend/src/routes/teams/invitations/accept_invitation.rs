use crate::models::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::Claims;
use actix_web::{post, web, HttpResponse};
use serde_json::json;

#[utoipa::path(
    responses(
        (status = 200, description = "Successfully joined team."),
        (status = 403, description = "User already belongs to team."),
        (status = 404, description = "Invitation not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams/invitations"
)]
#[post("/accept_invitation/{team_name}")]
pub async fn accept_invitation(
    app_state: web::Data<app_state::AppState>,
    claim_data: Claims,
    team_name: web::Path<String>,
) -> Result<HttpResponse, Error> {
    teams::Model::accept_invitation(&app_state.database, team_name.into_inner(), claim_data)
        .await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}
