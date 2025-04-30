use crate::entities::{teams, users};
use crate::models::task::RegistrationMode;
use crate::models::user::validate_name_chars;
use crate::routes::teams::AuthMiddleware;
use crate::routes::teams::TeamError;
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;
use actix_web::web::Json;
use actix_web::{HttpResponse, post, web};
use actix_web_validation::Validated;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateTeamModel {
    #[validate(length(min = 3, max = 32), custom(function = "validate_name_chars"))]
    pub team_name: String,
}

#[utoipa::path(
    request_body = CreateTeamModel,
    responses(
        (status = 200, description = "Team successfully created."),
        (status = 403, description = "User already belongs to team."),
        (status = 409, description = "Team already exists."),
        (status = 500, description = "Internal server error.")
    ),
    security(
        ("access_token" = [])
    ),
    tag = "teams"
)]
#[post("/create", wrap = "AuthMiddleware::with_user()")]
pub async fn create(
    app_state: web::Data<app_state::AppState>,
    Validated(create_team_model): Validated<Json<CreateTeamModel>>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let registration_config = app_state.task_manager.registration_config.lock().await;

    let now = Utc::now();
    if now < registration_config.start_date || now > registration_config.end_date {
        return Err(TeamError::InvalidRegistrationPeriod.into());
    }

    if let RegistrationMode::External = registration_config.registration_mode {
        return Err(TeamError::CannotRegisterInInternalMode.into());
    }

    if let Some(team) = user.get_team(&app_state.database).await? {
        return Err(TeamError::UserAlreadyBelongsToTeam {
            team_name: team.name,
        }
        .into());
    }

    if teams::Model::find_by_name(&app_state.database, &create_team_model.team_name)
        .await?
        .is_some()
    {
        return Err(TeamError::AlreadyExists.into());
    }

    teams::Model::create(
        &app_state.database,
        create_team_model.team_name.clone(),
        user,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
