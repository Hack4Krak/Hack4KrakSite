use crate::routes::teams::TeamError::TeamIsFull;
use crate::utils::error::Error;
use chrono::{DateTime, Duration, FixedOffset, Utc};
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

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
