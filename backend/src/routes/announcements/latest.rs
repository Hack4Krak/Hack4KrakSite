use crate::entities::announcement;
use crate::models::announcement::AnnouncementResponse;
use crate::routes::announcements::AnnouncementsError;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};
use sea_orm::{EntityTrait, QueryOrder};

#[utoipa::path(
    responses(
        (status = 200, description = "The latest announcement.", body = AnnouncementResponse),
        (status = 404, description = "No announcements found.")
    ),
    tag = "announcements"
)]
#[get("/latest")]
pub async fn latest(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let model = announcement::Entity::find()
        .order_by_desc(announcement::Column::CreatedAt)
        .one(&app_state.database)
        .await?
        .ok_or(AnnouncementsError::NoAnnouncementsFound)?;

    Ok(HttpResponse::Ok().json(model.response()?))
}
