use crate::entities::users;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{get, HttpResponse};
use sea_orm::EntityTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "User list successfully retrieved."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_users_list",
    tag = "admin/users"
)]
#[get("/list")]
pub async fn list(app_state: Data<AppState>) -> Result<HttpResponse, Error> {
    let users = users::Entity::find().all(&app_state.database).await?;

    Ok(HttpResponse::Ok().json(users))
}
