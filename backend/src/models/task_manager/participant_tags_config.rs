use crate::utils::error::Error;
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

const PARTICIPANT_TAGS_TYPE_VERIFIED: &str = "verified";

#[derive(Serialize, Deserialize, ToSchema, Default, Debug, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct ParticipantTagsConfig {
    pub participant_tags: Vec<ParticipantTag>,
}

#[derive(Serialize, Deserialize, ToSchema, Debug, Clone)]
#[serde(rename_all(deserialize = "kebab-case"))]
pub struct ParticipantTag {
    pub id: String,
    pub name: String,
    pub description: String,
    #[serde(rename = "type")]
    pub tag_type: String,
}

impl ParticipantTagsConfig {
    pub fn tag_by_id(&self, id: &str) -> Option<&ParticipantTag> {
        self.participant_tags.iter().find(|tag| tag.id == id)
    }
    pub fn tag_exists(&self, id: &str) -> bool {
        self.tag_by_id(id).is_some()
    }

    pub fn is_presence_verification_tag(&self, id: &str) -> Result<bool, Error> {
        let participant_tag = self.tag_by_id(id).ok_or(Error::InvalidTagId {
            tag_id: id.to_string(),
        })?;
        Ok(participant_tag.tag_type == PARTICIPANT_TAGS_TYPE_VERIFIED)
    }
}
