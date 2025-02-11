use actix_web::{get, web, HttpResponse};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

use crate::entities::users;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::JwtClaims;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserInformationResponse {
    pub username: String,
    pub email: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Use information received.", body = UserInformationResponse),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "user"
)]
#[get("/")]
pub async fn user(
    app_state: web::Data<app_state::AppState>,
    claim_data: JwtClaims,
) -> Result<HttpResponse, Error> {
    let user_model = users::Entity::find_by_id(claim_data.id)
        .one(&app_state.database)
        .await?
        .ok_or(Error::Unauthorized)?;

    Ok(HttpResponse::Ok().json(UserInformationResponse {
        email: user_model.email,
        username: user_model.username,
    }))
}

pub fn config(cfg: &mut utoipa_actix_web::service_config::ServiceConfig) {
    cfg.service(user);
}
