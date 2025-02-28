use crate::entities::{flag_capture, teams};
use crate::utils::error::Error;
use chrono::NaiveDateTime;
use sea_orm::{DatabaseConnection, EntityTrait};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::collections::HashMap;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct TeamPoints {
    pub label: String,
    pub points: Vec<usize>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TeamCurrentPoints {
    pub team_name: String,
    pub current_points: usize,
    pub captured_flags: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Default)]
pub struct TeamPointsAndFlagsOverTime {
    pub points: Vec<usize>,
    pub flags: Vec<usize>,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct Chart {
    pub event_timestamps: Vec<NaiveDateTime>,
    pub team_points_over_time: Vec<TeamPoints>,
}

#[derive(Default)]
pub struct PointsCounter {
    event_timestamps: Vec<NaiveDateTime>,
    team_points_and_flags_over_time: HashMap<String, TeamPointsAndFlagsOverTime>,
}

impl PointsCounter {
    pub async fn work(database: &DatabaseConnection) -> Result<PointsCounter, Error> {
        let captures = flag_capture::Entity::find().all(database).await?;
        let teams = teams::Entity::find().all(database).await?;

        let mut output = Self::default();
        let mut task_to_teams: HashMap<String, Vec<Uuid>> = HashMap::new();
        let mut team_to_tasks: HashMap<Uuid, Vec<String>> = HashMap::new();

        let mut task_points: HashMap<String, usize> = HashMap::new();

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
            }

            output.event_timestamps.push(capture.submitted_at);
        }

        Ok(output)
    }

    pub fn get_final_team_points(&self) -> Vec<TeamCurrentPoints> {
        let mut points = self
            .team_points_and_flags_over_time
            .iter()
            .map(|(team_name, points_and_flags)| {
                let final_points = *points_and_flags.points.last().unwrap_or(&0);
                let final_flags = *points_and_flags.flags.last().unwrap_or(&0);
                TeamCurrentPoints {
                    team_name: team_name.clone(),
                    current_points: final_points,
                    captured_flags: final_flags,
                }
            })
            .collect::<Vec<TeamCurrentPoints>>();

        points.sort_by_key(|team_current_points| team_current_points.current_points);
        points.reverse();

        points
    }

    pub fn to_chart(self) -> Chart {
        let points = self
            .team_points_and_flags_over_time
            .into_iter()
            .map(|(team_id, time_points_and_flags)| TeamPoints {
                label: team_id,
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
