use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceDescriptionUpdated {
        experience_id: ExperienceId,
        description: Description,
    },
    ExperienceSensationUpdated {
        experience_id: ExperienceId,
        sensation: SensationName,
    },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CreateExperienceRequest {
    pub agent: AgentName,
    pub sensation: SensationName,
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExperienceDescriptionRequest {
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpdateExperienceSensationRequest {
    pub sensation: SensationName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceRequests {
    CreateExperience(CreateExperienceRequest),
    UpdateExperienceDescription(UpdateExperienceDescriptionRequest),
    UpdateExperienceSensation(UpdateExperienceSensationRequest),
    GetExperience { id: ExperienceId },
    ListExperiences,
}
