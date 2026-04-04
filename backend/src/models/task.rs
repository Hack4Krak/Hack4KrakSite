use crate::routes::teams::TeamError::TeamIsFull;
use crate::utils::error::Error;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "kebab-case")]
pub enum EventStageType {
    /// When flag submission opens (point in time); exactly one required
    EventStart,
    /// When flag submission closes (point in time); exactly one required
    EventEnd,
    /// Info marker (point in time or range if end-date is provided)
    Informative,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct EventStage {
    pub name: String,
    #[serde(rename(deserialize = "type"))]
    pub stage_type: EventStageType,
    pub start_date: DateTime<FixedOffset>,
    pub end_date: Option<DateTime<FixedOffset>>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct EventConfig {
    pub id: String,
    pub name: String,
    pub stages: Vec<EventStage>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct RegistrationConfig {
    pub start_date: DateTime<FixedOffset>,
    pub end_date: DateTime<FixedOffset>,
    pub max_teams: u16,
    pub max_team_size: u16,
    pub registration_mode: RegistrationMode,
    pub max_teams_per_organization: u16,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct LabelsConfig {
    pub labels: Vec<Label>,
}

#[derive(Serialize, Deserialize, ToSchema, Default, Debug)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct Label {
    pub id: String,
    pub name: String,
    pub description: String,
}

#[derive(Debug, Serialize, Deserialize, ToSchema)]
#[serde(rename_all = "kebab-case")]
pub enum RegistrationMode {
    /// The users register teams themselves
    Internal,
    /// Teams are registered externally (e.g., by a supervisor, teacher, etc.)
    External,
}

impl RegistrationConfig {
    pub fn assert_team_size(&self, team_size: u16) -> Result<(), Error> {
        if team_size > self.max_team_size {
            return Err(Error::Team(TeamIsFull {
                max_size: self.max_team_size,
            }));
        }

        Ok(())
    }
}

impl Default for RegistrationConfig {
    fn default() -> Self {
        RegistrationConfig {
            start_date: DateTime::from(Utc::now() - Duration::minutes(10)),
            end_date: DateTime::from(Utc::now() + Duration::minutes(10)),
            max_teams: 100,
            max_team_size: 5,
            registration_mode: RegistrationMode::Internal,
            max_teams_per_organization: 3,
        }
    }
}

impl EventConfig {
    pub fn event_stage_start(
        &self,
        event_stage_type: EventStageType,
    ) -> Option<DateTime<FixedOffset>> {
        self.stage_by_type(&event_stage_type).map(|s| s.start_date)
    }

    pub fn is_before_event(&self) -> bool {
        let now = Utc::now();
        self.event_stage_start(EventStageType::EventStart)
            .map(|start| now < start)
            .unwrap_or(true)
    }

    pub fn is_during_event(&self) -> bool {
        let after_start = !self.is_before_event();
        let before_end = !self.is_after_event();
        after_start && before_end
    }

    pub fn is_after_event(&self) -> bool {
        let now = Utc::now();
        self.event_stage_start(EventStageType::EventEnd)
            .map(|end| now >= end)
            .unwrap_or(false)
    }

    pub fn stage_by_type(&self, event_stage_type: &EventStageType) -> Option<&EventStage> {
        if event_stage_type == &EventStageType::Informative {
            None
        } else {
            self.stages
                .iter()
                .find(|s| &s.stage_type == event_stage_type)
        }
    }

    pub fn stage_by_name(&self, name: &str) -> Option<&EventStage> {
        self.stages.iter().find(|s| s.name == name)
    }
}

impl Default for EventConfig {
    fn default() -> Self {
        EventConfig {
            id: "tasks".to_string(),
            name: "Default Event".to_string(),
            stages: vec![
                EventStage {
                    name: "Event Start".to_string(),
                    stage_type: EventStageType::EventStart,
                    start_date: DateTime::default(),
                    end_date: None,
                },
                EventStage {
                    name: "Event End".to_string(),
                    stage_type: EventStageType::EventEnd,
                    start_date: DateTime::default(),
                    end_date: None,
                },
            ],
        }
    }
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Default)]
pub struct TaskConfig {
    #[serde(flatten)]
    pub meta: TaskMeta,
    #[serde(skip_serializing)]
    pub flag_hash: String,
    #[serde(default)]
    pub assets: Vec<TaskAsset>,
    pub display: TaskDisplay,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct TaskMeta {
    pub id: String,
    pub name: String,
    pub labels: Vec<String>,
    pub difficulty_estimate: String,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct TaskDisplay {
    pub icon_coordinates: Coordinates,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone, Default)]
pub struct Coordinates {
    pub x: i32,
    pub y: i32,
}

#[derive(Serialize, Deserialize, ToSchema, Debug)]
pub struct TaskAsset {
    pub description: String,
    pub path: String,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_assert_team_size() {
        let config = RegistrationConfig {
            max_team_size: 5,
            ..Default::default()
        };

        assert!(config.assert_team_size(1).is_ok());
        assert!(config.assert_team_size(5).is_ok());
        assert!(config.assert_team_size(6).is_err());
    }
}
