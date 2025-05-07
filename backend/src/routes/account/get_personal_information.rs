use crate::AuthMiddleware;
use crate::entities::{user_personal_info, users};
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::Data;
use actix_web::{HttpResponse, get};

#[utoipa::path(
    responses(
        (status = 200, description = "User information received.", body = user_personal_info::Model),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    operation_id = "user_get_personal_information",
    tag = "account"
)]
#[get("/get_personal_information", wrap = "AuthMiddleware::with_user()")]
pub async fn get_personal_information(
    app_state: Data<app_state::AppState>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let personal_info =
        user_personal_info::Model::get_user_personal_information(user.id, &app_state.database)
            .await?;

    Ok(HttpResponse::Ok().json(personal_info))
}
