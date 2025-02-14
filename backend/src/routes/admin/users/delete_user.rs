use crate::entities::users;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{delete, HttpResponse};
use uuid::Uuid;

#[utoipa::path(
    responses(
        (status = 200, description = "User successfully deleted"),
        (status = 403, description = "User must have higher role than updated user."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/users"
)]
#[delete("/delete/{id}")]
pub async fn delete_user(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    id: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    users::Model::delete_user(&app_state.database, user, id.into_inner()).await?;

    Ok(HttpResponse::Ok().finish())
}
