use crate::routes::teams::AuthMiddleware;
use actix_web::{post, web, HttpResponse};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

use crate::entities::{teams, users};
use crate::routes::teams::TeamError::{AlreadyExists, UserAlreadyBelongsToTeam};
use crate::utils::app_state;
use crate::utils::error::Error;
use crate::utils::success_response::SuccessResponse;

#[derive(Serialize, Deserialize, ToSchema, Validate)]
pub struct CreateTeamModel {
    #[validate(length(min = 3, max = 32))]
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
    create_team_model: actix_web_validator::Json<CreateTeamModel>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    if let Some(team) = user.get_team(&app_state.database).await? {
        return Err(Error::Team(UserAlreadyBelongsToTeam {
            team_name: team.name,
        }));
    }

    if teams::Model::find_by_name(&app_state.database, &create_team_model.team_name)
        .await?
        .is_some()
    {
        return Err(Error::Team(AlreadyExists));
    }

    teams::Model::create(
        &app_state.database,
        create_team_model.team_name.clone(),
        user,
    )
    .await?;

    Ok(SuccessResponse::default().http_response())
}
