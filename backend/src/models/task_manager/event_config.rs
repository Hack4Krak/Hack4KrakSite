use chrono::{DateTime, FixedOffset, Utc};
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

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct EventConfig {
    pub id: String,
    pub name: String,
    pub stages: Vec<EventStage>,
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
