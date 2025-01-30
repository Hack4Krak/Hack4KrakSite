use crate::models::entities::teams;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::Claims;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use serde_json::json;
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct AddUserModel {
    pub username: String,
}

#[utoipa::path(
    request_body = AddUserModel,
    responses(
        (status = 200, description = "User successfully invited to team."),
        (status = 403, description = "User already belongs to team."),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = ["leader"])
    ),
    tag = "teams/management"
)]
#[post("/invite_user")]
pub async fn invite_user(
    app_state: web::Data<app_state::AppState>,
    add_user_model: web::Json<AddUserModel>,
    claim_data: Claims,
) -> Result<HttpResponse, Error> {
    teams::Model::invite_user(&app_state.database, add_user_model.0, claim_data).await?;

    Ok(HttpResponse::Ok().json(json!({
        "status": "ok"
    })))
}
