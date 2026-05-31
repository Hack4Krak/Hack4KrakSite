use crate::entities::{user_onboarding, users};
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "User onboarding answers received.", body = Option<user_onboarding::Model>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "admin_get_onboarding",
    tag = "admin/users"
)]
#[get("/onboarding/{id}")]
pub async fn get_onboarding(
    app_state: Data<app_state::AppState>,
    user_uuid: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_model = users::Entity::find_by_id(user_uuid.into_inner())
        .one(&app_state.database)
        .await?
        .ok_or(Error::UserNotFound)?;

    if let Some(onboarding_id) = user_model.onboarding {
        let onboarding = user_onboarding::Entity::find_by_id(onboarding_id)
            .one(&app_state.database)
            .await?;

        return Ok(HttpResponse::Ok().json(onboarding));
    }

    Ok(HttpResponse::Ok().json(None::<user_onboarding::Model>))
}
