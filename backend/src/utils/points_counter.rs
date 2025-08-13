use crate::entities::{flag_capture, teams};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, QueryOrder};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct TeamPoints {
    pub label: String,
    pub color: String,
    pub points: Vec<usize>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TeamCurrentPoints {
    pub team_name: String,
    pub current_points: usize,
    pub captured_flags: usize,
    pub color: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Default)]
pub struct TeamPointsAndFlagsOverTimeAndTeamColor {
    pub points: Vec<usize>,
    pub flags: Vec<usize>,
    pub color: String,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct Chart {
    pub event_timestamps: Vec<NaiveDateTime>,
    pub team_points_over_time: Vec<TeamPoints>,
}

#[derive(Default, Debug)]
pub struct PointsCounter {
    event_timestamps: Vec<NaiveDateTime>,
    team_points_and_flags_over_time: HashMap<String, TeamPointsAndFlagsOverTimeAndTeamColor>,
}

impl PointsCounter {
    pub async fn work(app_state: &Arc<AppState>) -> Result<PointsCounter, Error> {
        let captures = flag_capture::Entity::find()
            .order_by_asc(flag_capture::Column::SubmittedAt)
            .all(&app_state.database)
            .await?;
        let teams = teams::Entity::find().all(&app_state.database).await?;

        let mut output = Self::default();
        let mut task_to_teams: HashMap<String, Vec<Uuid>> = HashMap::new();
        let mut team_to_tasks: HashMap<Uuid, Vec<String>> = HashMap::new();

        let mut task_points: HashMap<String, usize> = HashMap::new();

        // TODO: Improve this
        output.event_timestamps.push(
            app_state
                .task_manager
                .event_config
                .read()
                .await
                .start_date
                .naive_utc(),
        );
        for team in &teams {
            output
                .team_points_and_flags_over_time
                .entry(team.name.clone())
                .or_default()
                .points
                .push(0);
        }

        for capture in captures {
            let task_name = capture.task.clone();
            let team_id = capture.team;

            task_to_teams
                .entry(task_name.clone())
                .or_default()
                .push(team_id);

            team_to_tasks
                .entry(team_id)
                .or_default()
                .push(task_name.clone());

            let solves_amount = task_to_teams[&task_name].len();
            let points = Self::calculate_task_points(solves_amount, teams.len());

            task_points.insert(task_name.clone(), points);

            for team in &teams {
                let solves = team_to_tasks.get(&team.id);

                let mut sum = 0;
                if let Some(solves) = solves {
                    for solve in solves {
                        sum += task_points.get(solve).unwrap();
                    }
                }

                output
                    .team_points_and_flags_over_time
                    .entry(team.name.clone())
                    .or_default()
                    .points
                    .push(sum);

                output
                    .team_points_and_flags_over_time
                    .entry(team.name.clone())
                    .or_default()
                    .flags
                    .push(solves.unwrap_or(&Vec::new()).len());

                output
                    .team_points_and_flags_over_time
                    .entry(team.name.clone())
                    .or_default()
                    .color = team.color.clone();
            }

            output.event_timestamps.push(capture.submitted_at);
        }

        Ok(output)
    }

    pub fn get_final_team_points(&self) -> Vec<TeamCurrentPoints> {
        let mut points = self
            .team_points_and_flags_over_time
            .iter()
            .map(|(team_name, points_and_flags_and_color)| {
                let final_points = *points_and_flags_and_color.points.last().unwrap_or(&0);
                let final_flags = *points_and_flags_and_color.flags.last().unwrap_or(&0);
                TeamCurrentPoints {
                    team_name: team_name.clone(),
                    current_points: final_points,
                    captured_flags: final_flags,
                    color: points_and_flags_and_color.color.clone(),
                }
            })
            .collect::<Vec<TeamCurrentPoints>>();

        points.sort_by_key(|team_current_points| team_current_points.current_points);
        points.reverse();

        points
    }

    pub fn team_rank(&self, team_name: &str) -> Option<(usize, usize)> {
        let team_points = self.get_final_team_points();
        let team_rank = team_points
            .iter()
            .position(|team| team.team_name == team_name)?
            + 1;

        Some((team_rank, team_points.len()))
    }

    pub fn to_chart(self) -> Chart {
        let points = self
            .team_points_and_flags_over_time
            .into_iter()
            .map(|(team_id, time_points_and_flags)| TeamPoints {
                label: team_id,
                color: time_points_and_flags.color,
                points: time_points_and_flags.points,
            })
            .collect::<Vec<TeamPoints>>();

        Chart {
            event_timestamps: self.event_timestamps,
            team_points_over_time: points,
        }
    }

    fn calculate_task_points(solves_amount: usize, total_teams: usize) -> usize {
        const DEFAULT_POINTS_PER_TASK: usize = 500;
        const POINTS_TO_DISTRIBUTE: usize = 400;

        if solves_amount <= 2 {
            return DEFAULT_POINTS_PER_TASK;
        }

        let solved_teams = (solves_amount - 2) as f64;

        let points = DEFAULT_POINTS_PER_TASK as f64
            - solved_teams * (POINTS_TO_DISTRIBUTE as f64 / (total_teams - 2) as f64);

        points.round() as usize
    }
}

impl Serialize for PointsCounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PointsCounter", 2)?;

        state.serialize_field("events", &self.event_timestamps)?;

        let team_points: Vec<_> = self
            .team_points_and_flags_over_time
            .iter()
            .map(|(team_name, points_and_flags)| {
                serde_json::json!({
                    "label": team_name,
                    "data": points_and_flags.points,
                })
            })
            .collect();

        state.serialize_field("team_points_over_time", &team_points)?;

        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_task_points_with_2_or_fewer_solves() {
        assert_eq!(PointsCounter::calculate_task_points(0, 3), 500);
        assert_eq!(PointsCounter::calculate_task_points(1, 3), 500);
        assert_eq!(PointsCounter::calculate_task_points(2, 3), 500);
    }

    #[test]
    fn test_calculate_task_points_with_more_solves() {
        let points = PointsCounter::calculate_task_points(3, 5);
        assert_eq!(points, 367, "Expected points to decrease with more solves");

        let points = PointsCounter::calculate_task_points(4, 5);
        assert_eq!(points, 233, "Expected points to decrease with more solves");
    }

    #[test]
    fn test_calculate_task_everybody_solved() {
        let points = PointsCounter::calculate_task_points(100, 100);
        assert_eq!(points, 100);
    }
}
