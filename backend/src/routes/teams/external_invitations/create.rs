use crate::entities::{email_verification_request, teams};
use crate::models::email_verification_request::EmailVerificationAction;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, post};
use actix_web_validation::Validated;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;
use validator::Validate;

pub const MAX_EXTERNAL_TEAMS_PER_ORGANIZATION: u64 = 3;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateExternalTeamModel {
    #[validate(length(min = 1, max = MAX_EXTERNAL_TEAMS_PER_ORGANIZATION))]
    pub teams: Vec<(String, u16)>,
}

#[utoipa::path(
    description = "Create external team invitations using secret token",
    request_body = CreateExternalTeamModel,
    responses(
        (status = 200, body = Vec<Vec<String>>),
        (status = 400),
        (status = 500)
    ),
    operation_id = "create_external",
    security(
        ("access_token" = [])
    ),
    tag = "teams/external_invitations"
)]
#[post("/create/{confirmation_code}")]
pub async fn create(
    app_state: Data<app_state::AppState>,
    Validated(Json(payload)): Validated<Json<CreateExternalTeamModel>>,
    confirmation_code: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    let registration_config = app_state.task_manager.registration_config.read().await;
    let confirmation_code = confirmation_code.into_inner();

    let email_verification_request =
        email_verification_request::Model::find_and_verify(&app_state.database, confirmation_code)
            .await?;
    let EmailVerificationAction::RegisterTeam { organization: _ } =
        email_verification_request.get_action()?
    else {
        return Err(Error::InvalidEmailConfirmationCode);
    };

    let mut team_invitation_codes = Vec::with_capacity(payload.teams.len());
    for (team_name, members_count) in payload.teams {
        let invitation_codes = teams::Model::create_as_external(
            &app_state.database,
            &registration_config,
            team_name.clone(),
            members_count,
            confirmation_code,
        )
        .await?;
        team_invitation_codes.push(invitation_codes);
    }

    Ok(HttpResponse::Ok().json(team_invitation_codes))
}
