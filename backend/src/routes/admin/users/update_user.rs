use crate::entities::users;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json, Path};
use actix_web::{patch, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct UpdateUserModel {
    pub username: Option<String>,
    pub email: Option<String>,
    pub team: Option<String>,
}

#[utoipa::path(
    request_body = UpdateUserModel,
    responses(
        (status = 200, description = "User successfully updated."),
        (status = 403, description = "User must have higher role than updated user."),
        (status = 500, description = "Internal server error.")
    ),
    tag = "admin/users"
)]
#[patch("/update/{id}")]
pub async fn update_user(
    app_state: Data<app_state::AppState>,
    user: users::Model,
    id: Path<Uuid>,
    update_user_json: Json<UpdateUserModel>,
) -> Result<HttpResponse, Error> {
    users::Model::update_user(
        &app_state.database,
        user,
        id.into_inner(),
        update_user_json.into_inner(),
    )
    .await?;

    Ok(HttpResponse::Ok().finish())
}
