use crate::entities::email_verification_request;
use crate::models::email_verification_request::EmailVerificationAction;
use crate::services::emails::{Email, EmailTemplate};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
pub struct RegisterExternalTeamModel {
    pub school: String,
    pub email_address: String,
}

#[utoipa::path(
    request_body = RegisterExternalTeamModel,
    responses(
        (status = 200, description = "Confirmation code successfully generated."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "admin/teams"
)]
#[post("/register_external_team")]
pub async fn register_external_team(
    app_state: Data<app_state::AppState>,
    payload: Json<RegisterExternalTeamModel>,
) -> Result<HttpResponse, Error> {
    let confirmation_code = email_verification_request::Model::create(
        &app_state.database,
        EmailVerificationAction::RegisterTeam {
            organization: payload.school.clone(),
        },
        payload.email_address.clone(),
        None,
    )
    .await?;

    // todo: link to frontend with code as parameter
    let confirmation_link = format!(
        "https://hack4krak.pl/admin/register_external_team?code={}",
        confirmation_code
    );
    Email {
        sender: (
            Some("Kontakt Hack4Krak".to_string()),
            "kontakt@hack4krak.pl".to_string(),
        ),
        recipients: vec![payload.email_address.to_string()],
        subject: "Rejestracja szkoły w CTF".to_string(),
        template: EmailTemplate::RegisterExternalTeam,
        placeholders: Some(vec![
            ("school".to_string(), payload.school.clone()),
            ("link".to_string(), confirmation_link.to_string()),
        ]),
    }
    .send(&app_state)
    .await?;

    Ok(SuccessResponse::default().http_response())
}
