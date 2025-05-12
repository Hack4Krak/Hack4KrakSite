use crate::entities::users;
use crate::services::emails;
use crate::services::emails::{Email, EmailMeta};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use sea_orm::QueryFilter;
use sea_orm::{ColumnTrait, EntityTrait, QuerySelect};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct EmailSendingModel {
    #[serde(flatten)]
    pub meta: EmailMeta,
    pub address: String,
    pub send_target: EmailSendTarget,
    pub content: String,
}

#[derive(Serialize, Deserialize, ToSchema)]
pub enum EmailSendTarget {
    AllUsers,
    SpecificUsernames(Vec<String>),
    SpecificEmails(Vec<String>),
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
#[post("/send_informational")]
pub async fn send_informational(
    app_state: Data<app_state::AppState>,
    model: Json<EmailSendingModel>,
) -> Result<HttpResponse, Error> {
    let recipients = match model.send_target {
        EmailSendTarget::AllUsers => {
            users::Entity::find()
                .select_only()
                .column(users::Column::Email)
                .into_tuple()
                .all(&app_state.database)
                .await?
        }
        EmailSendTarget::SpecificUsernames(usernames) => {
            users::Entity::find()
                .filter(users::Column::Username.is_in(usernames))
                .select_only()
                .column(users::Column::Email)
                .into_tuple()
                .all(&app_state.database)
                .await?
        }
        EmailSendTarget::SpecificEmails(emails) => {
            // Use the provided specific emails
            emails.clone()
        }
    };

    Email::new_with_meta(
        &model.address,
        recipients,
        Box::new(emails::Informational {
            title: model.meta.subject.clone(),
            content: model.content.clone(),
        }),
        Some(model.meta.clone()),
    )
    .send(&app_state.smtp_client)
    .await?;

    Ok(SuccessResponse::default().http_response())
}
