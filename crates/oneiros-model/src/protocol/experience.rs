use serde::{Deserialize, Serialize};

use crate::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectExperienceById {
    pub id: ExperienceId,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ListExperiencesFilter {
    #[serde(default)]
    pub agent: Option<AgentName>,
    #[serde(default)]
    pub sensation: Option<SensationName>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceDescriptionUpdate {
    pub experience_id: ExperienceId,
    pub description: Description,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExperienceSensationUpdate {
    pub experience_id: ExperienceId,
    pub sensation: SensationName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceEvents {
    ExperienceCreated(Experience),
    ExperienceDescriptionUpdated(ExperienceDescriptionUpdate),
    ExperienceSensationUpdated(ExperienceSensationUpdate),
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
    GetExperience(SelectExperienceById),
    ListExperiences(ListExperiencesFilter),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case", tag = "type", content = "data")]
pub enum ExperienceResponses {
    ExperienceCreated(Experience),
    ExperienceUpdated(Experience),
    ExperienceFound(Experience),
    ExperiencesListed(Vec<Experience>),
}
