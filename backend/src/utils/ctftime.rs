use crate::entities::{flag_capture, teams};
use crate::services::task_manager::TaskManager;
use crate::utils::error::Error;
use crate::utils::points_counter::PointsCounter;
use sea_orm::EntityTrait;
use sea_orm::{ColumnTrait, DatabaseConnection};
use sea_orm::{QueryFilter, QueryOrder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use utoipa::ToSchema;

/// Capture log event for https://ctftime.org/json-scoreboard-feed
#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
pub struct CaptureLogEvent {
    pub id: i32,
    pub time: Option<i64>,
    pub r#type: Option<String>,
    pub team: String,
    pub victim: Option<String>,
    pub task: Option<String>,
    #[serde(rename = "pointsDelta")]
    pub points_delta: Option<usize>,
}

/// Team Standings for https://ctftime.org/json-scoreboard-feed
#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct TeamStandings {
    pub tasks: Vec<String>,
    pub standings: Vec<SingleTeamStanding>,
}

pub async fn get_capture_log(
    db: &DatabaseConnection,
    last_id: Option<i32>,
) -> Result<Vec<CaptureLogEvent>, Error> {
    let last_id = last_id.unwrap_or(0);

    let captures: Vec<(flag_capture::Model, Option<teams::Model>)> = flag_capture::Entity::find()
        .filter(flag_capture::Column::Id.gt(last_id))
        .order_by_asc(flag_capture::Column::Id)
        .find_also_related(teams::Entity)
        .all(db)
        .await?;

    captures
        .into_iter()
        .map(|(capture, team)| {
            let team = team.ok_or(Error::MissingTeamForFlagCapture {
                capture_id: capture.id,
            })?;
            Ok(CaptureLogEvent {
                id: capture.id,
                time: Some(capture.submitted_at.and_utc().timestamp()),
                r#type: Some("taskCorrect".to_string()),
                team: team.name,
                task: Some(capture.task),
                points_delta: None,
                ..Default::default()
            })
        })
        .collect()
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct SingleTeamStanding {
    pub team: String,
    pub score: usize,
    #[serde(rename = "taskStats")]
    pub task_stats: HashMap<String, TaskTeamStat>,
    #[serde(rename = "lastAccept")]
    pub last_accept: i64,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct TaskTeamStat {
    pub points: usize,
    pub time: i64,
}

impl PointsCounter {
    pub async fn to_ctftime_standings(
        &self,
        db: &DatabaseConnection,
        task_manager: &TaskManager,
    ) -> Result<TeamStandings, Error> {
        let tasks_by_team =
            teams::Model::get_tasks_by_team(db, self.team_time_series.keys().copied().collect())
                .await?;

        let mut task_solve_counts: HashMap<&str, usize> = HashMap::new();
        let mut task_points: HashMap<String, usize> = HashMap::new();
        let total_teams = self.team_time_series.len();
        for task_map in tasks_by_team.values() {
            for task_name in task_map.keys() {
                *task_solve_counts.entry(task_name).or_default() += 1;
            }
        }
        for task in task_manager.tasks.iter() {
            let solves = *task_solve_counts.get(task.key().as_str()).unwrap_or(&0);
            task_points.insert(
                task.key().clone(),
                Self::calculate_task_value(solves, total_teams),
            );
        }

        let standings = self
            .team_time_series
            .iter()
            .map(|(uuid, data)| {
                let mut last_accept: i64 = 0;
                let task_stats = tasks_by_team
                    .get(uuid)
                    .map(|task_map| {
                        task_map
                            .iter()
                            .filter_map(|(task_name, submitted_at)| {
                                let time = submitted_at.and_utc().timestamp();
                                if time > last_accept {
                                    last_accept = time
                                }
                                task_points.get(task_name).map(|points| {
                                    (
                                        task_name.clone(),
                                        TaskTeamStat {
                                            points: *points,
                                            time,
                                        },
                                    )
                                })
                            })
                            .collect()
                    })
                    .unwrap_or_default();

                SingleTeamStanding {
                    team: data.name.clone(),
                    score: *data.points.last().unwrap_or(&0),
                    task_stats,
                    last_accept,
                }
            })
            .collect();

        let mut tasks: Vec<String> = task_manager
            .tasks
            .iter()
            .map(|entry| entry.key().clone())
            .collect();
        tasks.sort();
        Ok(TeamStandings { tasks, standings })
    }
}
