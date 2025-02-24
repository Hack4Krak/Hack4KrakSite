use crate::entities::email_confirmation;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "Email confirmations successfully fetched."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/users"
)]
#[get("/email_confirmations")]
pub async fn email_confirmations_list(
    app_state: Data<app_state::AppState>,
) -> Result<HttpResponse, Error> {
    let email_confirmations = email_confirmation::Entity::find()
        .all(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(email_confirmations))
}
