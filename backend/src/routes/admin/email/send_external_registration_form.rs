use crate::entities::email_verification_request;
use crate::models::email_verification_request::EmailVerificationAction;
use crate::services::emails;
use crate::services::env::EnvConfig;
use crate::utils::app_state;
use crate::utils::email::Email;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct ExternalRegistrationFormModel {
    pub organization: String,
    pub email_address: String,
}

#[utoipa::path(
    request_body = ExternalRegistrationFormModel,
    responses(
        (status = 200, description = "Confirmation code successfully generated."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/email"
)]
#[post("/send_external_registration_form")]
pub async fn send_external_registration_form(
    app_state: Data<app_state::AppState>,
    payload: Json<ExternalRegistrationFormModel>,
) -> Result<HttpResponse, Error> {
    let confirmation_code = email_verification_request::Model::create(
        &app_state.database,
        EmailVerificationAction::RegisterTeam {
            organization: payload.organization.clone(),
        },
        payload.email_address.clone(),
        None,
    )
    .await?;

    let mut url = EnvConfig::get().frontend_url.clone();
    url.set_path("admin/register_external_team");
    url.query_pairs_mut()
        .append_pair("code", &confirmation_code.to_string());

    Email::new(
        "kontakt",
        vec![payload.email_address.to_string()],
        Box::new(emails::ExternalRegistrationForm {
            link: url.to_string(),
            organization: payload.organization.clone(),
        }),
    )
    .send(&app_state.smtp_client)
    .await?;

    Ok(SuccessResponse::default().http_response())
}
