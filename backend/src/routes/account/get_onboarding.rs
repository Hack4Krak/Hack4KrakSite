use crate::entities::{user_onboarding, users};
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "User onboarding answers received.", body = Option<user_onboarding::Model>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "get_onboarding",
    tag = "account"
)]
#[get("/onboarding")]
pub async fn get_onboarding(
    app_state: Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    if let Some(onboarding_id) = user.onboarding {
        let onboarding = user_onboarding::Entity::find_by_id(onboarding_id)
            .one(&app_state.database)
            .await?
            .ok_or(Error::Unauthorized)?;
        return Ok(HttpResponse::Ok().json(onboarding));
    }

    Ok(HttpResponse::Ok().json(None::<user_onboarding::Model>))
}
