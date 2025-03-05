use crate::services::emails::{Email, EmailTemplate};
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EmailSendingModel {
    pub sender: (Option<String>, String),
    /// If recipients is None, the email will be sent to all users.
    pub recipients: Option<Vec<String>>,
    pub subject: String,
    pub template: EmailTemplate,
    pub placeholders: Option<Vec<(String, String)>>,
}

#[utoipa::path(
    request_body = EmailSendingModel,
    responses(
        (status = 200, description = "Email successfully sent"),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/email"
)]
#[post("/send")]
pub async fn send(
    app_state: Data<app_state::AppState>,
    model: Json<EmailSendingModel>,
) -> Result<HttpResponse, Error> {
    Email::from_admin_sending_model(&app_state.database, model.into_inner())
        .await?
        .send(&app_state)
        .await?;

    Ok(HttpResponse::Ok().finish())
}
