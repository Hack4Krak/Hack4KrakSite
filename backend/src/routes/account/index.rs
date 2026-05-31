use crate::entities::users;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::jwt::JwtClaims;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UserInformationResponse {
    pub username: String,
    pub first_name: Option<String>,
    pub email: String,
    pub has_completed_onboarding: bool,
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
#[get("/")]
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
        first_name: user_model.first_name,
        has_completed_onboarding: user_model.onboarding.is_some(),
    }))
}
