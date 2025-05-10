use crate::entities::{email_verification_request, external_team_invitation};
use crate::models::email_verification_request::EmailVerificationAction;
use crate::models::external_team_invitation::TeamCodes;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Path};
use actix_web::{HttpResponse, get};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema)]
struct ExternalInvitationsInfo {
    pub organization: String,
    pub is_registered: bool,
    pub teams: Vec<TeamCodes>,
}

#[utoipa::path(
    description = "Create external team invitations using secret token",
    responses(
        (status = 200, body = ExternalInvitationsInfo),
        (status = 400),
        (status = 500)
    ),
    operation_id = "external_invitations_info",
    security(
        ("access_token" = [])
    ),
    tag = "teams/external_invitations"
)]
#[get("/info/{code}")]
pub async fn info(
    app_state: Data<app_state::AppState>,
    confirmation_code: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let confirmation_code = confirmation_code.into_inner();

    let request =
        email_verification_request::Model::find_and_verify(&app_state.database, confirmation_code)
            .await?;
    let EmailVerificationAction::RegisterTeam { organization } = request.get_action()? else {
        return Err(Error::InvalidEmailConfirmationCode);
    };

    let teams =
        external_team_invitation::Model::grouped_codes(&app_state.database, confirmation_code)
            .await?;

    Ok(HttpResponse::Ok().json(ExternalInvitationsInfo {
        organization,
        is_registered: !teams.is_empty(),
        teams,
    }))
}
