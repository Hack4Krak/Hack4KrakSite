use crate::entities::{email_verification_request, teams};
use crate::models::email_verification_request::EmailVerificationAction;
use crate::routes::teams::TeamError;
use crate::utils::app_state;
use crate::utils::error::Error;
use actix_web::web::{Data, Json, Path};
use actix_web::{HttpResponse, post};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use uuid::Uuid;

pub const MAX_EXTERNAL_TEAMS: usize = 3;

#[derive(Debug, Serialize, Deserialize, ToSchema)]
pub struct CreateExternalTeamModel {
    pub teams: Vec<(String, u8)>,
}

#[utoipa::path(
    description = "Create external team invitations using secret token",
    request_body = CreateExternalTeamModel,
    responses(
        (status = 200),
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
    Json(payload): Json<CreateExternalTeamModel>,
    confirmation_code: Path<Uuid>,
) -> Result<HttpResponse, Error> {
    if payload.teams.is_empty() || payload.teams.len() > MAX_EXTERNAL_TEAMS {
        return Err(TeamError::InvalidNumberOfTeams {
            max_teams: MAX_EXTERNAL_TEAMS,
        }
        .into());
    }

    let confirmation_code = confirmation_code.into_inner();

    let email_verification_request =
        email_verification_request::Model::find_and_verify(&app_state.database, confirmation_code)
            .await?;
    let EmailVerificationAction::RegisterTeam { organization: _ } =
        email_verification_request.get_action()?
    else {
        return Err(Error::InvalidEmailConfirmationCode);
    };

    let mut team_invitation_codes = vec![];

    for team in payload.teams {
        let invitation_codes = teams::Model::create_with_external(
            &app_state.database,
            team.0.clone(),
            team.1,
            confirmation_code,
        )
        .await?;
        team_invitation_codes.push(invitation_codes);
    }

    email_verification_request
        .delete(&app_state.database)
        .await?;

    Ok(HttpResponse::Ok().json(team_invitation_codes))
}
