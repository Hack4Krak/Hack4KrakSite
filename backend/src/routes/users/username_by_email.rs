use crate::entities::users;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use serde::Deserialize;
use utoipa::ToSchema;

#[derive(Deserialize, ToSchema)]
pub struct UsernameByEmailRequest {
    pub email: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Username found.", body = String),
        (status = 404, description = "User not found."),
        (status = 500, description = "Internal server error.")
    ),
    operation_id = "username_by_email",
    tag = "users"
)]
#[post("/username-by-email")]
pub async fn username_by_email(
    app_state: Data<app_state::AppState>,
    request_body: Json<UsernameByEmailRequest>,
) -> Result<HttpResponse, Error> {
    let user_model = users::Model::find_by_email(&app_state.database, &request_body.email)
        .await?
        .ok_or(Error::UserNotFound)?;

    Ok(HttpResponse::Ok().json(user_model.username))
}
