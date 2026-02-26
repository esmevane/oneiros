use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceRefAdded {
        experience_id: ExperienceId,
        record_ref: RecordRef,
        created_at: Timestamp,
    },
    ExperienceDescriptionUpdated {
        experience_id: ExperienceId,
        description: Description,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExperienceRequest {
    pub agent: AgentName,
    pub sensation: SensationName,
    pub description: Description,
    #[serde(default)]
    pub refs: Vec<RecordRef>,
}

/// A request to add a reference to an experience.
///
/// Accepts either form via serde untagged:
///   Identified: { "id": "...", "kind": "cognition", "role": "origin" }
///   Linked:     { "link": "base64url...", "role": "origin" }
pub type AddExperienceRefRequest = RecordRef;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExperienceDescriptionRequest {
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceRequests {
    CreateExperience(CreateExperienceRequest),
    AddExperienceRef(AddExperienceRefRequest),
    UpdateExperienceDescription(UpdateExperienceDescriptionRequest),
    GetExperience { id: ExperienceId },
    ListExperiences,
}
