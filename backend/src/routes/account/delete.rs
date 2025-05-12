use crate::entities::users;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::Data;
use actix_web::{HttpResponse, delete};
use sea_orm::ModelTrait;

#[utoipa::path(
    responses(
        (status = 200, description = "Account successfully deleted"),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "account"
)]
#[delete("/delete")]
pub async fn delete(
    app_state: Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    user.delete(&app_state.database).await?;

    Ok(SuccessResponse::default().http_response())
}
