use crate::entities::{user_personal_info, users};
use crate::routes::account::AccountError;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "User information received.", body = user_personal_info::Model),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "admin_get_personal_information",
    tag = "admin/users"
)]
#[get("/get_personal_information/{id}")]
pub async fn get_personal_information(
    app_state: Data<app_state::AppState>,
    id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let user_model = users::Entity::find_by_id(id.into_inner())
        .one(&app_state.database)
        .await?
        .ok_or(Error::Unauthorized)?;

    if let Some(personal_info_id) = user_model.personal_info {
        let personal_info = user_personal_info::Entity::find_by_id(personal_info_id)
            .one(&app_state.database)
            .await?
            .ok_or(Error::Unauthorized)?;
        return Ok(HttpResponse::Ok().json(personal_info));
    }

    Err(Error::Account(AccountError::UserDoesNotHavePersonalInfo))
}
