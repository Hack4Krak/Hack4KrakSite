use crate::entities::users;
use crate::entities::users::UpdatableModel;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, patch};
use sea_orm::EntityTrait;
use uuid::Uuid;

#[utoipa::path(
    request_body = UpdatableModel,
    responses(
        (status = 200, description = "User successfully updated."),
        (status = 403, description = "User must have higher role than updated user."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "admin_users_update",
    tag = "admin/users"
)]
#[patch("/update/{id}")]
pub async fn update(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    id: Path<Uuid>,
    update_user_json: Json<UpdatableModel>,
) -> Result<HttpResponse, Error> {
    let updated_user = users::Entity::find_by_id(*id)
        .one(&app_state.database)
        .await?
        .ok_or(Error::UserNotFound)?;

    let updatable_user_model = update_user_json.into_inner();

    users::Model::update_as_admin(
        &app_state.database,
        user,
        updated_user,
        updatable_user_model,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
