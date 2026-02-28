use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceRefAdded {
        experience_id: ExperienceId,
        experience_ref: ExperienceRef,
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
    pub refs: Vec<ExperienceRef>,
}

/// A request to add a reference to an experience.
///
/// Accepts: { "entity": "base64url-ref-string", "role": "origin" }
pub type AddExperienceRefRequest = ExperienceRef;

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
