use crate::entities::{user_personal_info, users};
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "User personal information received.", body = Option<user_personal_info::Model>),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "user_get_personal_information",
    tag = "account"
)]
#[get("/get_personal_information")]
pub async fn get_personal_information(
    app_state: Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    if let Some(personal_info_id) = user.personal_info {
        let personal_info = user_personal_info::Entity::find_by_id(personal_info_id)
            .one(&app_state.database)
            .await?
            .ok_or(Error::Unauthorized)?;
        return Ok(HttpResponse::Ok().json(personal_info));
    }

    Ok(HttpResponse::Ok().json(None::<user_personal_info::Model>))
}
