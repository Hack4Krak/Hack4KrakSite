use crate::entities::{flag_capture, teams, users};
use crate::routes::flag::FlagError;
use crate::routes::teams::TeamError;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::error::Error::{Flag, Team};
use crate::utils::server_event::{
    BroadcastEnvelope, LeaderboardUpdateEvent, ServerEvent, TeamFlagCaptureEvent,
};
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use actix_web_validation::Validated;
use sea_orm::EntityTrait;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
struct SubmitModel {
    #[validate(length(max = 1024))]
    pub flag: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema, Validate)]
struct SubmitResponse {
    pub task_id: String,
}

#[utoipa::path(
    responses(
        (status = 200, description = "Correctly submitted flag.", body = SubmitResponse),
        (status = 400, description = "Invalid flag"),
        (status = 403, description = "Team is not verified for flag submission"),
        (status = 409, description = "Flag already submitted by the team"),
    ),
    tag = "flag"
)]
#[post("/submit")]
pub async fn submit(
    app_state: Data<AppState>,
    model: Validated<Json<SubmitModel>>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let flag = model
        .flag
        .strip_prefix("hack4KrakCTF{")
        .and_then(|value| value.strip_suffix("}"));

    let Some(flag) = flag else {
        return Err(FlagError::InvalidFlagFormat.into());
    };

    let task = app_state
        .task_manager
        .find_by_flag(flag)
        .ok_or(Flag(FlagError::InvalidFlag))?;

    let event_config = app_state.task_manager.event_config.read().await;

    if !event_config.is_after_event() {
        let Some(team_id) = user.team else {
            return Err(Team(TeamError::UserDoesntBelongToAnyTeam {
                username: user.username,
            }));
        };

        let team = teams::Entity::find_by_id(team_id)
            .one(&app_state.database)
            .await?
            .ok_or(Team(TeamError::TeamNotFound))?;

        team.assert_is_confirmed()?;

        let is_first_submission = !flag_capture::Model::completed(
            &app_state.database,
            team.clone(),
            task.key().to_string(),
        )
        .await?;

        app_state.invalidate_points_cache().await;

        let task_id = task.key().to_string();
        let task_name = task.value().meta.name.to_string();

        let _ = app_state.server_event_sender.send(BroadcastEnvelope::public(
            ServerEvent::LeaderboardUpdate(LeaderboardUpdateEvent {
                task_id: task_id.clone(),
                task_name: task_name.clone(),
                is_first_flag_submission: is_first_submission,
                team_name: team.name,
                username: user.username.clone(),
            }),
        ));

        let _ = app_state.server_event_sender.send(BroadcastEnvelope::team(
            team.id,
            ServerEvent::TeamFlagCapture(TeamFlagCaptureEvent {
                task_id,
                task_name,
                username: user.username,
                is_first_flag_submission: is_first_submission,
            }),
        ));
    }

    Ok(HttpResponse::Ok().json(SubmitModel {
        flag: task.key().clone(),
    }))
}
