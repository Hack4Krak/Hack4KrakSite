use crate::entities::{flag_capture, teams};
use crate::utils::app_state::AppState;
use crate::utils::error::Error;
use chrono::NaiveDateTime;
use sea_orm::{EntityTrait, QueryOrder};
use serde::ser::SerializeStruct;
use serde::{Deserialize, Serialize, Serializer};
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use utoipa::ToSchema;
use uuid::Uuid;

#[derive(Debug, Default, Clone)]
struct TeamTimeSeriesData {
    points: Vec<usize>,
    color: String,
    solve_timestamps: Vec<NaiveDateTime>,
    current_flags: usize,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct TeamPointsTimeSeries {
    pub name: String,
    pub color: String,
    pub points: Vec<usize>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, PartialEq)]
pub struct TeamRanking {
    pub team_name: String,
    pub current_points: usize,
    pub captured_flags: usize,
    pub color: String,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, PartialEq)]
pub struct LeaderboardChart {
    pub event_timestamps: Vec<NaiveDateTime>,
    pub team_points_over_time: Vec<TeamPointsTimeSeries>,
}

#[derive(Default, Debug, Clone)]
pub struct PointsCounter {
    event_timestamps: Vec<NaiveDateTime>,
    team_time_series: HashMap<String, TeamTimeSeriesData>,
}

impl PointsCounter {
    /// Calculates points for all teams based on flag captures.
    /// Uses caching to avoid redundant database queries.
    pub async fn calculate(app_state: &Arc<AppState>) -> Result<PointsCounter, Error> {
        {
            let cache = app_state.points_cache.read().await;
            if let Some(cached) = cache.as_ref() {
                return Ok(cached.clone());
            }
        }

        let captures = flag_capture::Entity::find()
            .order_by_asc(flag_capture::Column::SubmittedAt)
            .order_by_asc(flag_capture::Column::Id)
            .all(&app_state.database)
            .await?;

        let teams = teams::Entity::find().all(&app_state.database).await?;
        let start_time = app_state
            .task_manager
            .event_config
            .read()
            .await
            .start_date
            .naive_utc();

        let mut counter = Self::default();
        counter.event_timestamps.push(start_time);

        for team in &teams {
            counter.team_time_series.insert(
                team.name.clone(),
                TeamTimeSeriesData {
                    points: vec![0],
                    color: team.color.clone(),
                    solve_timestamps: vec![],
                    current_flags: 0,
                },
            );
        }

        counter.process_events(captures, &teams);

        *app_state.points_cache.write().await = Some(counter.clone());

        Ok(counter)
    }

    fn process_events(&mut self, captures: Vec<flag_capture::Model>, teams: &[teams::Model]) {
        let mut task_solve_counts: HashMap<String, usize> = HashMap::new();
        let mut team_solves: HashMap<Uuid, HashSet<String>> = HashMap::new();
        let total_teams = teams.len();

        for capture in captures {
            *task_solve_counts.entry(capture.task.clone()).or_default() += 1;

            team_solves
                .entry(capture.team)
                .or_default()
                .insert(capture.task.clone());

            for team in teams {
                let solved_tasks = team_solves.get(&team.id);

                let current_points = match solved_tasks {
                    Some(tasks) => tasks
                        .iter()
                        .map(|t_id| {
                            Self::calculate_task_value(
                                *task_solve_counts.get(t_id).unwrap_or(&0),
                                total_teams,
                            )
                        })
                        .sum(),
                    None => 0,
                };

                let flags_count = solved_tasks.map(|s| s.len()).unwrap_or(0);

                let team_data = self.team_time_series.get_mut(&team.name).unwrap();
                team_data.points.push(current_points);
                team_data.current_flags = flags_count;

                if team.id == capture.team {
                    team_data.solve_timestamps.push(capture.submitted_at);
                }
            }

            self.event_timestamps.push(capture.submitted_at);
        }
    }

    /// Calculates point value for a task based on solve count using linear decay.
    /// Points range from 500 (max) to 100 (min), decreasing linearly as more teams solve it.
    fn calculate_task_value(solve_count: usize, total_teams: usize) -> usize {
        const MAX_POINTS: f64 = 500f64;
        const MIN_POINTS: f64 = 100f64;
        const DECAY_RANGE: f64 = MAX_POINTS - MIN_POINTS;

        if total_teams <= 2 || solve_count <= 2 {
            return MAX_POINTS as usize;
        }

        let decay_factor = DECAY_RANGE / (total_teams - 2) as f64;
        let solves_active = (solve_count - 2) as f64;
        let points = MAX_POINTS - (solves_active * decay_factor);

        points.round().max(MIN_POINTS) as usize
    }

    pub fn get_rankings(&self) -> Vec<TeamRanking> {
        let mut rankings: Vec<TeamRanking> = self
            .team_time_series
            .iter()
            .map(|(name, data)| TeamRanking {
                team_name: name.clone(),
                current_points: *data.points.last().unwrap_or(&0),
                captured_flags: data.current_flags,
                color: data.color.clone(),
            })
            .collect();

        rankings.sort_by(|a, b| {
            let points_cmp = b.current_points.cmp(&a.current_points);
            if points_cmp != Ordering::Equal {
                return points_cmp;
            }

            let history_a = &self.team_time_series[&a.team_name].solve_timestamps;
            let history_b = &self.team_time_series[&b.team_name].solve_timestamps;

            for (time_a, time_b) in history_a.iter().rev().zip(history_b.iter().rev()) {
                match time_a.cmp(time_b) {
                    Ordering::Equal => continue,
                    ordering => return ordering,
                }
            }

            match history_a.len().cmp(&history_b.len()) {
                Ordering::Equal => a.team_name.cmp(&b.team_name),
                ordering => ordering.reverse(),
            }
        });

        rankings
    }

    pub fn team_rank(&self, team_name: &str) -> Option<(usize, usize)> {
        let rankings = self.get_rankings();
        let position = rankings.iter().position(|r| r.team_name == team_name)?;
        Some((position + 1, rankings.len()))
    }

    pub fn to_chart(self) -> LeaderboardChart {
        let team_points_over_time = self
            .team_time_series
            .into_iter()
            .map(|(name, data)| TeamPointsTimeSeries {
                name,
                color: data.color,
                points: data.points,
            })
            .collect();

        LeaderboardChart {
            event_timestamps: self.event_timestamps,
            team_points_over_time,
        }
    }
}

impl Serialize for PointsCounter {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("PointsCounter", 2)?;
        state.serialize_field("events", &self.event_timestamps)?;

        let series: Vec<_> = self
            .team_time_series
            .iter()
            .map(|(k, v)| {
                serde_json::json!({
                    "label": k,
                    "data": v.points
                })
            })
            .collect();

        state.serialize_field("team_points_over_time", &series)?;
        state.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculate_task_points_with_2_or_fewer_solves() {
        assert_eq!(PointsCounter::calculate_task_value(0, 3), 500);
        assert_eq!(PointsCounter::calculate_task_value(1, 3), 500);
        assert_eq!(PointsCounter::calculate_task_value(2, 3), 500);
    }

    #[test]
    fn test_calculate_task_points_with_more_solves() {
        let points = PointsCounter::calculate_task_value(3, 5);
        assert_eq!(points, 367, "Expected points to decrease with more solves");

        let points = PointsCounter::calculate_task_value(4, 5);
        assert_eq!(points, 233, "Expected points to decrease with more solves");
    }

    #[test]
    fn test_calculate_task_everybody_solved() {
        let points = PointsCounter::calculate_task_value(100, 100);
        assert_eq!(points, 100);
    }

    #[test]
    fn test_calculate_task_value_never_below_minimum() {
        for solve_count in 0..200 {
            let points = PointsCounter::calculate_task_value(solve_count, 50);
            assert!(
                points >= 100,
                "Points should never fall below minimum (100)"
            );
            assert!(points <= 500, "Points should never exceed maximum (500)");
        }
    }

    #[test]
    fn test_calculate_task_value_linear_decay() {
        let total_teams = 10;
        let points_3_solves = PointsCounter::calculate_task_value(3, total_teams);
        let points_4_solves = PointsCounter::calculate_task_value(4, total_teams);
        let points_5_solves = PointsCounter::calculate_task_value(5, total_teams);

        assert!(points_3_solves > points_4_solves);
        assert!(points_4_solves > points_5_solves);

        let decay_1 = points_3_solves - points_4_solves;
        let decay_2 = points_4_solves - points_5_solves;
        assert_eq!(decay_1, decay_2, "Decay should be linear");
    }
}
