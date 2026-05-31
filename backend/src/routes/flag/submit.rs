use crate::TaskReleasePhaseMiddleware;
use crate::entities::{flag_capture, teams, users};
use crate::routes::flag::FlagError;
use crate::routes::teams::TeamError;
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use crate::utils::error::Error::{Flag, Team};
use crate::utils::points_counter::PointsCounter;
use crate::utils::sse_event::SseEvent;
use actix_web::web::{Data, Json};
use actix_web::{HttpResponse, post};
use actix_web_validation::Validated;
use sea_orm::{ColumnTrait, EntityTrait, PaginatorTrait, QueryFilter};
use serde::{Deserialize, Serialize};
use tracing::{error, info, warn};
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
    pub task_title: String,
    pub points: usize,
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
#[post("/submit", wrap = "TaskReleasePhaseMiddleware")]
pub async fn submit(
    app_state: Data<AppState>,
    model: Validated<Json<SubmitModel>>,
    user: users::Model,
) -> Result<HttpResponse, Error> {
    let username = user.username.clone();
    let team_id = user.team;
    let team_name = match team_id {
        Some(team_id) => teams::Entity::find_by_id(team_id)
            .one(&app_state.database)
            .await?
            .map(|team| team.name),
        None => None,
    };

    let flag = model
        .flag
        .strip_prefix("hack4KrakCTF{")
        .and_then(|value| value.strip_suffix("}"));

    let Some(flag) = flag else {
        warn!(
            target: "flag_submit",
            %username,
            team_name,
            "Rejected flag submission: invalid format"
        );
        return Err(FlagError::InvalidFlagFormat.into());
    };

    let task = match app_state.task_manager.find_by_flag(flag) {
        Some(task) => task,
        None => {
            warn!(
                target: "flag_submit",
                %username,
                team_name,
                submitted_flag = flag,
                "Rejected flag submission: invalid flag"
            );
            return Err(Flag(FlagError::InvalidFlag));
        }
    };

    let event_config = app_state.task_manager.event_config.read().await;

    if !event_config.is_after_event() {
        let Some(team_id) = team_id else {
            return Err(Team(TeamError::UserDoesntBelongToAnyTeam {
                username: user.username,
            }));
        };

        let team = teams::Entity::find_by_id(team_id)
            .one(&app_state.database)
            .await?
            .ok_or(Team(TeamError::TeamNotFound))?;

        team.assert_is_confirmed()?;

        let task_id = task.key().to_string();
        let task_name = task.value().meta.name.to_string();
        let team_name = team.name.clone();

        let is_first_submission = match flag_capture::Model::completed(
            &app_state.database,
            team,
            task_id.clone(),
        )
        .await
        {
            Ok(is_first_submission) => !is_first_submission,
            Err(Error::Flag(FlagError::AlreadySubmittedFlag)) => {
                warn!(
                    target: "flag_submit",
                    %username,
                    %team_name,
                    %task_id,
                    task_name,
                    "Rejected flag submission: already submitted"
                );
                return Err(Flag(FlagError::AlreadySubmittedFlag));
            }
            Err(err) => return Err(err),
        };

        info!(
            target: "flag_submit",
            %username,
            %team_name,
            %task_id,
            task_name,
            "Accepted flag submission"
        );

        let solve_count = flag_capture::Entity::find()
            .filter(flag_capture::Column::Task.eq(task_id.clone()))
            .count(&app_state.database)
            .await? as usize;
        let team_count = teams::Entity::find().count(&app_state.database).await? as usize;
        let points = PointsCounter::calculate_task_value(solve_count, team_count);

        app_state.invalidate_points_cache().await;

        if let Err(err) = app_state
            .sse_event_sender
            .send(SseEvent::LeaderboardUpdate {
                task_id: task_id.clone(),
                task_name: task_name.clone(),
                is_first_flag_submission: is_first_submission,
                team_name,
                username,
            })
        {
            error!("Failed to broadcast leaderboard update: {err}");
        }

        return Ok(HttpResponse::Ok().json(SubmitResponse {
            task_id,
            task_title: task_name,
            points,
        }));
    }

    let task_id = task.key().to_string();
    let task_name = task.value().meta.name.to_string();

    info!(
        target: "flag_submit",
        %username,
        team_name,
        %task_id,
        task_name,
        "Accepted flag submission after event"
    );

    Ok(HttpResponse::Ok().json(SubmitResponse {
        task_id,
        task_title: task_name,
        points: 0,
    }))
}
